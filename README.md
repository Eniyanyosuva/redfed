# RedFed 🔴

**A decentralized, censorship-resistant social platform built on Solana**

RedFed is a Web3 social platform that enables fast, transparent, and immutable community discussions powered by blockchain technology. Every thread, vote, and interaction lives on-chain, ensuring true ownership and freedom of expression.

![Platform](https://img.shields.io/badge/Platform-Solana-blueviolet)
![License](https://img.shields.io/badge/License-MIT-green)
![Status](https://img.shields.io/badge/Status-Prototype-orange)

## 🌟 Features

### Core Functionality
- **Decentralized Threads** - Create censorship-resistant discussions stored on-chain
- **Community Voting** - Upvote/downvote system for community-driven content curation
- **Web3 Identity** - Connect with Solana wallets (Phantom, Solflare, etc.)
- **Real-time Updates** - Instant interactions leveraging Solana's speed
- **Transparent Governance** - All votes and content immutably recorded on blockchain

### Technical Highlights
- ⚡ **Fast Transactions** - Sub-second confirmation times on Solana
- 💰 **Low Fees** - Fraction-of-a-cent costs per interaction
- 🔒 **Censorship-Resistant** - No central authority can delete or modify content
- 🌐 **Truly Decentralized** - Your data, your ownership
- 📊 **On-Chain Analytics** - Transparent metrics and community statistics

## 🚀 Getting Started

### Prerequisites
- Node.js (v16 or higher)
- A Solana wallet (Phantom, Solflare, etc.)
- Basic understanding of Web3 concepts

### Installation

```bash
# Clone the repository
git clone https://github.com/eniyanyosuva/redfed.git

# Navigate to project directory
cd redfed

# Install dependencies
npm install

# Start development server
npm run dev
```

### Quick Start

1. **Connect Your Wallet**
   - Click "Connect Wallet" in the top right
   - Approve the connection in your wallet extension

2. **Browse Threads**
   - View trending or new discussions
   - See community votes and engagement

3. **Create Content**
   - Click "Create Thread"
   - Write your title and content
   - Post to the blockchain

4. **Engage**
   - Upvote quality content
   - Downvote spam or low-quality posts
   - Watch the community curate itself

## 🏗️ Architecture

### Frontend Stack
- **React** - UI framework
- **Tailwind CSS** - Styling and design system
- **Lucide React** - Icon library

### Blockchain Integration (Production)
- **Solana Web3.js** - Blockchain interaction
- **Anchor Framework** - Smart contract development
- **Wallet Adapter** - Multi-wallet support

### Smart Contract Structure
```
programs/
├── redfed/
│   ├── src/
│   │   ├── lib.rs           # Main program logic
│   │   ├── state.rs         # Account structures
│   │   └── instructions.rs  # Program instructions
│   └── Cargo.toml
```

## 📝 Core Concepts

### Thread Structure
Each thread is stored on-chain with:
- Unique thread ID
- Author's wallet address
- Title and content
- Upvote/downvote counts
- Timestamp
- Reply count

### Voting Mechanism
- Users can upvote or downvote once per thread
- Votes are recorded on-chain
- Score = Upvotes - Downvotes
- Voting requires connected wallet

### Content Ranking
- **Trending**: Sorted by vote score (upvotes - downvotes)
- **New**: Sorted by timestamp (most recent first)

## 🔧 Development

### Project Structure
```
redfed/
├── src/
│   ├── components/       # React components
│   ├── hooks/           # Custom React hooks
│   ├── utils/           # Helper functions
│   └── App.jsx          # Main application
├── programs/            # Solana smart contracts
├── tests/              # Test files
└── public/             # Static assets
```

### Running Tests
```bash
# Run frontend tests
npm test

# Run Solana program tests
anchor test
```

### Building for Production
```bash
# Build frontend
npm run build

# Build Solana program
anchor build
```

## 🌐 Deployment

### Deploy Smart Contracts
```bash
# Deploy to devnet
anchor deploy --provider.cluster devnet

# Deploy to mainnet
anchor deploy --provider.cluster mainnet
```

### Deploy Frontend
```bash
# Build production bundle
npm run build

# Deploy to Vercel/Netlify/etc.
vercel deploy
```

## 🔐 Security Considerations

- All user interactions require wallet signatures
- Content is immutable once posted
- Vote manipulation is prevented by wallet-based authentication
- No central database vulnerable to attacks
- Open-source and auditable smart contracts

## 🛣️ Roadmap

### Phase 1: Core Platform ✅
- [x] Thread creation and display
- [x] Voting system
- [x] Wallet integration
- [x] Basic UI/UX

### Phase 2: Enhanced Features 🚧
- [ ] Reply/comment system
- [ ] User profiles and reputation
- [ ] Thread categories/tags
- [ ] Search functionality
- [ ] Media attachments (IPFS integration)

### Phase 3: Advanced Features 📋
- [ ] Token-gated communities
- [ ] NFT avatars
- [ ] Tip/reward system with native token
- [ ] Governance voting
- [ ] Cross-chain bridges

### Phase 4: Ecosystem Growth 🌱
- [ ] Mobile app (iOS/Android)
- [ ] Desktop application
- [ ] API for third-party integrations
- [ ] Developer SDK

## 💡 Use Cases

- **Community Forums** - Decentralized discussion boards
- **DAO Governance** - Proposal discussions and voting
- **Content Creator Hubs** - Direct audience engagement
- **News Aggregation** - Community-curated news feeds
- **Research Collaboration** - Open scientific discourse

## 🤝 Contributing

We welcome contributions! Here's how you can help:

1. Fork the repository
2. Create a feature branch (`git checkout -b feature/AmazingFeature`)
3. Commit your changes (`git commit -m 'Add some AmazingFeature'`)
4. Push to the branch (`git push origin feature/AmazingFeature`)
5. Open a Pull Request

### Contribution Guidelines
- Follow existing code style
- Write tests for new features
- Update documentation
- Be respectful and inclusive

## 📄 License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## 🙏 Acknowledgments

- Solana Foundation for blockchain infrastructure
- Anchor Framework team for development tools
- Web3 community for inspiration and support

## 📞 Contact & Community

- **Website**: https://redfed.io
- **Twitter**: @RedFedSocial
- **Discord**: discord.gg/redfed
- **Email**: hello@redfed.io

## ⚠️ Disclaimer

This is a prototype demonstration. Always conduct your own research and use caution when interacting with blockchain applications. Never share your private keys or seed phrases.

---

**Built with ❤️ on Solana**

*RedFed - Your voice, your chain, your community*