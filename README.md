# AMM(Automated Market Maker)

## Overview
This is a fully backed Automated Market Maker (AMM) built using the Anchor framework on Solana.
An AMM is a type of decentralized exchange (DEX) used in cryptocurrency trading. Instead of using a traditional order book (where buyers and sellers are matched), AMMs use algorithms to automatically set prices based on the supply and demand of assets in a liquidity pool.

---

## Installation

### Steps to Install

1. **Clone the repository:**
   ```sh
   git clone <your-repo-url>
   cd <your-repo-name>
   ```

2. **Install dependencies:**
   ```sh
   yarn install
   ```


3. **Build the Anchor program:**
   ```sh
   anchor build
   ```

4. **Deploy the program locally:**
   ```sh
   anchor deploy
   ```

5. **Run tests:**
   ```sh
   anchor test
   ```
---

## Constant Product AMM
Constant product AMM (automated market maker) is a decentralized exchange where 2 tokens are traded.

## Overview of AMM program

<div>
 <img src="Images/architecture.png" alt="Maths">
 <p> Hear Admin can create multiple Pools and user can swap or deposite there Tokens </p>
</div>

---

## Constant Product Curve

<div>
 <img src="Images/pool.png" alt="Maths">
</div>


---

## Swapping Token

<div>
 <img src="Images/swap-tokens.png" alt="Maths">
</div>


---

## Adding Liquidity to the Pool

<div>
 <img src="Images/add-liq.png" alt="Maths">
</div>


---

## Minting Tokens After Adding Liquidity

<div>
 <img src="Images/lp-mint-tokens.png" alt="Maths">
</div>


---

## Removing/Withdrawing Asset from Liquidity

<div>
 <img src="Images/remove-liq.png" alt="Maths">
</div>


---

## Contributing

We welcome contributions!
Contributions are what make the open source community such an amazing place to learn, inspire, and create. Any contributions you make are greatly appreciated.
If you have a suggestion that would make this better, please fork the repo and create a pull request. You can also simply open an issue with the tag "enhancement". Don't forget to give the project a star! Thanks again!:

1. Fork the repository.
2. Create a new branch (`git checkout -b feature/your-feature-name`).
3. Make your changes and commit them (`git commit -am 'Add new feature'`).
4. Push your branch (`git push origin feature/your-feature-name`).
5. Open a Pull Request to the main repository.

---

## References I have taken:-

- [AMM Blog Post](https://www.infect3d.xyz/blog/Exploring-AMMs) - This Blog post is really amazing easy to digest/understand the AMM in Technical POV,Written by [@InfectedCrypto](https://x.com/InfectedCrypto).
- [Constant Product Equation](https://youtu.be/QNPyFs8Wybk?si=TlaNLr0reoL3_S5S) - This video by [@ProgrammerSmart](https://x.com/ProgrammerSmart), he explained Mathmatical part for building AMM using constant product equation.
