/**
 * useStopwatch.ts
 *
 * Backend-authoritative stopwatch hook.
 *
 * Design principles (as per the refactor requirement):
 *  - React NEVER stores elapsed seconds as the source of truth.
 *  - On mount: calls GET /stopwatch/current to reconstruct state from server timestamps.
 *  - Clock display is computed locally: accumulatedSeconds + (Date.now()/1000 - segmentStart)
 *  - All signals (start/pause/resume/stop/split) go through the backend.
 *  - Page navigation, refresh, browser close, or interval failure cannot lose the session.
 */

import { useState, useEffect, useRef, useCallback } from 'react';
import { stopwatchApi, StopwatchState } from '../api/stopwatchApi';

export type SwStatus = 'idle' | 'running' | 'paused';

export interface UseStopwatchReturn {
  /** Current lifecycle status */
  status: SwStatus;
  /** Displayed elapsed milliseconds — derived from server timestamps, updated locally every 10ms */
  elapsedMs: number;
  /** Active session ID (null when idle) */
  sessionId: number | null;
  /** Current session label */
  label: string;
  /** Splits JSON string from server */
  splitsJson: string | null;
  /** Whether any async action is in-flight */
  isLoading: boolean;
  /** Last error message */
  error: string | null;

  // ── Actions ─────────────────────────────────────────────────────────────
  start: (label?: string, sessionDate?: string) => Promise<void>;
  pause: () => Promise<void>;
  resume: () => Promise<void>;
  stop: () => Promise<void>;
  addSplit: (note?: string) => Promise<void>;
  updateLabel: (label: string) => Promise<void>;
}

/** Convert server StopwatchState to local elapsedMs. Pure calculation, no side effects. */
function computeElapsedMs(serverState: StopwatchState): number {
  if (serverState.state === 'running' && serverState.segmentStartedAt) {
    const segmentStartUtcMs = new Date(serverState.segmentStartedAt).getTime();
    const elapsedInSegmentMs = Date.now() - segmentStartUtcMs;
    return serverState.accumulatedSeconds * 1000 + Math.max(0, elapsedInSegmentMs);
  }
  // paused or stopped: fixed value
  return serverState.accumulatedSeconds * 1000;
}

export function useStopwatch(): UseStopwatchReturn {
  const [status, setStatus]       = useState<SwStatus>('idle');
  const [elapsedMs, setElapsedMs] = useState(0);
  const [sessionId, setSessionId] = useState<number | null>(null);
  const [label, setLabel]         = useState('');
  const [splitsJson, setSplitsJson] = useState<string | null>(null);
  const [isLoading, setIsLoading] = useState(false);
  const [error, setError]         = useState<string | null>(null);

  // Ref holds the latest server state so the tick interval can read it without stale closure
  const serverStateRef = useRef<StopwatchState | null>(null);
  const tickRef        = useRef<ReturnType<typeof setInterval> | null>(null);

  // ── Tick management ──────────────────────────────────────────────────────

  const startTick = useCallback(() => {
    stopTick();
    tickRef.current = setInterval(() => {
      if (serverStateRef.current?.state === 'running') {
        setElapsedMs(computeElapsedMs(serverStateRef.current));
      }
    }, 30); // 30ms updates milliseconds smoothly
  }, []);

  function stopTick() {
    if (tickRef.current) {
      clearInterval(tickRef.current);
      tickRef.current = null;
    }
  }

  // ── Apply server state to local React state ──────────────────────────────

  const applyState = useCallback((s: StopwatchState | null) => {
    if (!s || s.state === 'stopped') {
      serverStateRef.current = null;
      setStatus('idle');
      setElapsedMs(0);
      setSessionId(null);
      setLabel('');
      setSplitsJson(null);
      stopTick();
      return;
    }

    serverStateRef.current = s;
    setSessionId(s.id);
    setLabel(s.label ?? '');
    setSplitsJson(s.splits ?? null);
    setElapsedMs(computeElapsedMs(s));

    if (s.state === 'running') {
      setStatus('running');
      startTick();
    } else {
      setStatus('paused');
      stopTick();
    }
  }, [startTick]);

  // ── Mount: reconstruct from server ───────────────────────────────────────

  useEffect(() => {
    let cancelled = false;
    (async () => {
      try {
        const current = await stopwatchApi.getCurrent();
        if (!cancelled) applyState(current);
      } catch {
        // Network error on mount — start idle, don't block UI
        if (!cancelled) applyState(null);
      }
    })();
    return () => {
      cancelled = true;
      stopTick();
    };
  }, [applyState]);

  // ── Generic action wrapper ───────────────────────────────────────────────

  const withLoading = useCallback(async (fn: () => Promise<StopwatchState | null>) => {
    setIsLoading(true);
    setError(null);
    try {
      const result = await fn();
      applyState(result);
    } catch (e: unknown) {
      // Try to extract the most descriptive error message
      let msg = 'Lỗi không xác định';
      if (typeof e === 'string') {
        msg = e;
      } else if (e instanceof Error) {
        msg = e.message;
      } else if (e && typeof e === 'object') {
        const objErr = e as { message?: string; error?: string };
        msg = objErr.error || objErr.message || 'Lỗi không xác định';
      }
      setError(msg);
    } finally {
      setIsLoading(false);
    }
  }, [applyState]);

  // ── Actions ──────────────────────────────────────────────────────────────

  const start = useCallback((lbl?: string, sDate?: string) => withLoading(() => stopwatchApi.start({ label: lbl, sessionDate: sDate })), [withLoading]);
  const pause  = useCallback(() => withLoading(() => stopwatchApi.pause()),  [withLoading]);
  const resume = useCallback(() => withLoading(() => stopwatchApi.resume()), [withLoading]);
  const stop   = useCallback(() => withLoading(() => stopwatchApi.stop()),   [withLoading]);
  const addSplit = useCallback((note?: string) => withLoading(() => stopwatchApi.split({ note })), [withLoading]);
  const updateLabel = useCallback((lbl: string) => withLoading(() => stopwatchApi.updateLabel(lbl)), [withLoading]);

  return {
    status, elapsedMs, sessionId, label, splitsJson, isLoading, error,
    start, pause, resume, stop, addSplit, updateLabel,
  };
}
