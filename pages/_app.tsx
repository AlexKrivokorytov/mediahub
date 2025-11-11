import type { AppProps } from 'next/app';
import { useEffect, useState } from 'react';
import { useRouter } from 'next/router';
import '../styles/globals.css';
import { AppShell } from '../components/layout/AppShell';
import { tauriService } from '../services/tauri';

export default function MyApp({ Component, pageProps }: AppProps) {
  const router = useRouter();
  const [handledOAuth, setHandledOAuth] = useState(false);

  useEffect(() => {
    if (handledOAuth) return;
    if (typeof window === 'undefined') return;
    const params = new URLSearchParams(window.location.search);
    const code = params.get('code');
    if (!code) return;
    setHandledOAuth(true);
    tauriService.spotifyCompleteAuth(code)
      .then(() => router.replace('/settings'))
      .catch(() => router.replace('/settings'));
  }, [handledOAuth, router]);

  return (
    <AppShell>
      <Component {...pageProps} />
    </AppShell>
  );
}
