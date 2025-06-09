# Battalion 

## 🧰 Tech Stack

![Rust](https://img.shields.io/badge/Rust-000000?style=for-the-badge&logo=rust&logoColor=white)
![SurrealDB](https://img.shields.io/badge/SurrealDB-ff0040?style=for-the-badge&logo=databricks&logoColor=white)

This project is a minimal web application built using **Rust (Axum)** and **SurrealDB**.

📚 **Documentation:**

## Quick Start

### 🛠 Installation & Setup

1. Clone the Repo 
```bash
git clone git@github.com:matthewharwood/Battalion.git
cd Battalion
```

2. Install Rust (if not already)
```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

3. Install SurrealDB (if not already)
```bash
curl -sSf https://install.surrealdb.com | sh
```

4. Start SurrealDB (in-memory)
```bash
surreal start --log trace --user root --pass root memory
```

5. Run the App
```bash
cargo run
```

Your app will be running on: [http://localhost:6969](http://localhost:6969)

### Service Ports Configuration

| Service     | Default Port |
| ----------- | ------------ |
| Backend     | 6969         |
| Database    | 8000         |


A CRM-like platform to form a battalion of cracked engineers and designers. 

### Design

https://www.figma.com/design/MNbbfknT22njnVnzXA75wz/Battalion?node-id=1-2&t=zjVcLXCFi04DTDh4-1


