import type { Metadata } from 'next';

export const metadata: Metadata = {
  title: 'CryptoCats',
  description: 'Claim your on-chain CryptoCats NFT airdrops',
};

export default function RootLayout({ children }: { children: React.ReactNode }) {
  return (
    <html lang="en">
      <body style={{ margin: 0, fontFamily: 'Arial, sans-serif', background: '#08070d', color: '#f5f5f5' }}>
        {children}
      </body>
    </html>
  );
}
