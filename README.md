# VISCA Control UI

🚧 **Status**: Work in Progress 🚧

## Overview
VISCA Control UI is a web application designed to interface with and control VISCA-compatible devices.

### Project Structure
- **Frontend**: Built with [SvelteKit](https://svelte.dev/)
- **Backend**: Built with [Rocket.rs](https://rocket.rs/)

## Requirements
To run this project locally, make sure you have the following installed:
- [**Node.js** and **npm**](https://nodejs.org/): Required for managing frontend dependencies.
- [**Rust**](https://www.rust-lang.org/): Required for compiling and running the backend server.

---

## Installation
Make sure the **Requirements** are installed before continuing.
### 1. Clone the Repository
```bash
git clone ...
cd VISCA-Control-UI
```
### 2. Setup Frontend
Navigate to the `frontend` folder and install the dependencies:
```bash
cd frontend
npm install
```
### 2. Setup Backend
Navigate to the `backend` folder and install the dependencies:
```bash
cd ../backend
cargo build
```

---

## Running the Application
### Frontend
Navigate to the `frontend` folder and run the development server:
```bash
cd frontend
npm run dev
```
The frontend will be available at [http://localhost:5173](http://localhost:5173)
### Backend
Navigate to the `backend` folder and run the backend server:
```bash
cd ../backend
cargo run
```
The backend will be reachable at [http://127.0.0.1:8000](http://127.0.0.1:8000). The frontend will communicate with the backend API.

---

## License
All rights reserved. This project is proprietary and confidential. Unauthorized copying, distribution, or modification of this project, via any medium, is strictly prohibited without prior written consent from the author.

For permissions, please contact the repository owner.
