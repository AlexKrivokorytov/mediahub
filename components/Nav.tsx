import Link from 'next/link';
import React from 'react';
import { useRouter } from 'next/router';

export default function Nav() {
  const { pathname } = useRouter();
  const items = [
    { href: '/', label: 'Home' },
    { href: '/music', label: 'Music' },
    { href: '/anime', label: 'Anime' },
    { href: '/discord', label: 'Discord' },
    { href: '/settings', label: 'Settings' },
  ];
  return (
    <nav className="px-5 pb-3 border-b border-mh-border">
      <div className="flex gap-2">
        {items.map((it) => {
          const active = pathname === it.href;
          return (
            <Link
              key={it.href}
              href={it.href}
              className={[
                'px-2.5 py-2 rounded-lg border text-sm transition',
                active
                  ? 'bg-mh-surface border-mh-border text-mh-text'
                  : 'bg-transparent border-transparent text-mh-muted hover:text-mh-text hover:bg-mh-elev hover:border-mh-border',
              ].join(' ')}
            >
              {it.label}
            </Link>
          );
        })}
      </div>
    </nav>
  );
}
