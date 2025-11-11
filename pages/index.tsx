import React from 'react';
import NowPlaying from '../components/NowPlaying';

export default function Home() {
  return (
    <div className="grid grid-cols-12 gap-4">
      <div className="col-span-12">
        <h1 className="text-2xl font-semibold mb-2">Welcome</h1>
        <p className="muted">Manage and play media from Spotify, SoundCloud, and track anime via Shikimori.</p>
      </div>
      <div className="col-span-12">
        <NowPlaying />
      </div>
    </div>
  );
}
