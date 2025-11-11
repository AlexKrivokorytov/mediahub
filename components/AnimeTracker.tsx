import React, { useState } from 'react';
import { tauriService, AnimeEntry } from '../services/tauri';
import { List } from 'lucide-react';

export default function AnimeTracker() {
  const [list, setList] = useState<AnimeEntry[]>([]);
  const [loading, setLoading] = useState(false);
  const [error, setError] = useState<string | null>(null);

  const load = async () => {
    setLoading(true); setError(null);
    try { setList(await tauriService.shikimoriGetAnimeList()); }
    catch (e: any) { setError(String(e)); }
    finally { setLoading(false); }
  };

  return (
    <section className="card">
      <div className="flex gap-2 mb-3">
        <button className="btn" onClick={load} disabled={loading}><List size={16} className="mr-1" /> {loading ? 'Loading...' : 'Load Anime List'}</button>
      </div>
      {error && <p className="text-red-500">{error}</p>}
      {list.length > 0 && (
        <ul className="space-y-2">
          {list.map((a) => (
            <li key={a.id} className="flex items-center justify-between rounded-xl border border-mh-border bg-mh-surface/60 px-3 py-2">
              <span>
                {a.title}
                <span className="muted"> — {a.episodes_watched}/{a.total_episodes} — {a.status} {a.rating ? `(${a.rating})` : ''}</span>
              </span>
            </li>
          ))}
        </ul>
      )}
      {list.length === 0 && !loading && !error && <p className="muted">No anime loaded yet.</p>}
    </section>
  );
}
