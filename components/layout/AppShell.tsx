import React from 'react';
import Nav from '../Nav';
import { Headphones } from 'lucide-react';

type Props = { children: React.ReactNode };

export function AppShell({ children }: Props) {
  return (
    <div className="min-h-full flex">
      <div className="flex-1 flex flex-col">
        <header className="sticky top-0 z-10 backdrop-blur bg-gradient-to-b from-black/70 to-black/30 border-b border-mh-border">
          <div className="flex items-center justify-between gap-3 px-5 py-3">
            <div className="flex items-center gap-2 font-semibold tracking-tight">
              <span className="inline-flex items-center justify-center w-7 h-7 rounded-lg bg-gradient-to-br from-mh-purple to-mh-orange text-black"><Headphones size={16} /></span>
              Media Hub
            </div>
            <div className="muted text-xs">Cross‑platform desktop player</div>
          </div>
          <Nav />
        </header>
        <div className="container-mh py-5">
          {children}
        </div>
      </div>
    </div>
  );
}
