import Image from 'next/image';

const traits = [
  { id: 0, name: 'Tabby', rarity: 'Common' },
  { id: 1, name: 'Shadow', rarity: 'Rare' },
  { id: 2, name: 'Galaxy', rarity: 'Legendary' },
];

export default function HomePage() {
  return (
    <main style={{ maxWidth: 940, margin: '0 auto', padding: '3rem 1.5rem' }}>
      <header style={{ display: 'flex', justifyContent: 'space-between', alignItems: 'center', marginBottom: '2rem' }}>
        <h1 style={{ fontSize: '2.5rem', margin: 0 }}>CryptoCats</h1>
        <a
          href="https://x.com/CTCatsfun"
          target="_blank"
          rel="noopener noreferrer"
          style={{
            display: 'inline-flex',
            alignItems: 'center',
            justifyContent: 'center',
            width: '44px',
            height: '44px',
            borderRadius: '999px',
            background: '#ffffff',
            color: '#08070d',
            textDecoration: 'none',
            fontWeight: 700,
            fontSize: '1.1rem',
            boxShadow: '0 6px 20px rgba(255,255,255,0.12)',
          }}
          aria-label="Visit CryptoCats on X"
        >
          X
        </a>
      </header>
      <p style={{ color: '#bdb8d8', lineHeight: 1.6 }}>
        Hold $CATS to qualify for randomized on-chain cat NFTs with rarity-driven traits.
      </p>
      <section style={{ marginTop: '2rem', display: 'grid', gap: '1.25rem' }}>
        {traits.map((trait) => (
          <div key={trait.id} style={{ border: '1px solid #2f2648', borderRadius: 16, padding: '1rem 1.25rem', background: '#141020' }}>
            <strong>{trait.name}</strong>
            <div style={{ color: '#8f8bad', marginTop: '0.4rem' }}>{trait.rarity} trait tier</div>
          </div>
        ))}
      </section>
      <section style={{ marginTop: '2rem' }}>
        <h2 style={{ fontSize: '1.25rem' }}>Collection view</h2>
        <p style={{ color: '#bdb8d8' }}>Connect your wallet to inspect your claim status and available airdrops.</p>
      </section>
    </main>
  );
}
