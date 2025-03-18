# **Yuumi - League of Legends Discord Bot for Item Builder**

### **How It Works**
1. A user sends a command like `/build gnar` or `!build gnar` in a Discord server.
2. Yumii queries the **Mistral AI agent** (temperature: `0.01`, model: `NeMo`) with the provided champion name.
3. The API returns an optimal item build formatted as:   `BUILD -> Doran's Ring > Hextech Gunblade > Nashor's Tooth > Rabadon's Deathcap > Void Staff > Zhonya's Hourglass`

### Tech Stack
- **Rust** 🦀
- **Poise** (Discord framework)
- **Tokio** (Async runtime)
- **Reqwest** (HTTP requests)
- **Serde** (JSON parsing)
- **Mistral AI** (LLM API for processing builds)
  - API URL: https://api.mistral.ai/v1/agents/completions
  - Model : NeMo
  - Max Tokens: 150

### Installation
1. clone the repo 
2. create .env file with `DISCORD_TOKEN` and `MISTRAL_API_KEY`
3. `cargo run`

### Contribution
Contributions are what make the open source community such an amazing place to learn, inspire, and create. Any contributions you make are **greatly appreciated**.

If you have a suggestion that would make this better, please fork the repo and create a pull request. You can also simply open an issue with the tag "enhancement".
Don't forget to give the project a star! Thanks again! ❤️

1. Fork the Project
2. Create your Feature Branch (`git checkout -b feature/AmazingFeature`)
3. Commit your Changes (`git commit -m 'Add some AmazingFeature'`)
4. Push to the Branch (`git push origin feature/AmazingFeature`)
5. Open a Pull Request

<p align="right">(<a href="#top">back to top</a>)</p>



