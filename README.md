A high-performance, full-stack catalog management system built with **Rust (Axum)** and **Vue 3 (Composition API)**. It features a distributed-ready backend using **libSQL**, strict type safety with **TypeScript** and **Zod**, and is fully containerized using **Podman** for seamless local development and production deployment.

---

## Technical Stack

### **Backend (The "Reliability" Layer)**
* **Language:** Rust (Edition 2024).
* **Framework:** **Axum** for high-performance, asynchronous routing.
* **Database:** **libSQL** (SQLite fork) utilizing remote connections for serverless-ready state management.
* **Security:** **Argon2** for password hashing and **JWT** (JSON Web Tokens) for stateless authentication.
* **Image Processing:** Native **WebP/JPEG/PNG** encoding for optimized asset delivery.

### **Frontend (The "Reactive" Layer)**
* **Environment:** **Deno** (Runtime) + **Vite** (Build Tool).
* **Framework:** **Vue 3** with the Composition API.
* **State Management:** **Pinia** for modular and reactive global state.
* **Type Safety:** Strict **TypeScript** integration and **Zod** for schema-based validation.
* **Styling:** **Tailwind CSS** + **DaisyUI** for a modern, responsive UI.

---

## System Architecture

The project follows a decoupled, container-first architecture managed via a `justfile` and orchestrated for development with **Zellij**.

* **ETL & Schema:** Automatically applies SQL schemas on startup, ensuring data consistency across environments.
* **Graceful Shutdown:** Implements signal handling for clean terminations in containerized environments (Kubernetes/Podman).
* **CORS Configuration:** Dynamic origin matching based on environment variables to support multiple client deployments.

---

## Local Development Workflow

### **Prerequisites**
* **Podman** (Daemonless container engine).
* **Just** (Command runner).
* **Zellij** (Terminal workspace manager - optional).

### **Commands**
1.  **Start Workspace:** Launches Neovim and terminal panes for both backend and frontend.
    ```bash
    just enter
    ```
2.  **Run Backend (Development):** Runs an Alpine-based Rust container with cached volumes for fast compilation.
    ```bash
    just run-backend
    ```
3.  **Run Frontend:**
    ```bash
    just run-frontend
    ```

---

## Deployment
The project uses a multi-stage **Containerfile** to produce a minimal production image.

* **Build Image:** `just build-backend`
* **Push to Registry:** `just upload-backend-image`

---

## Environment Variables
The following keys are required in your `.env`:
* **Backend:** `DB_URL`, `DB_TOKEN`, `SECRET_KEY`, `ADMIN_USER`, `ADMIN_PASSWORD`.
* **Frontend:** `VITE_API_URL`, `VITE_STORE_NAME`, `VITE_STORE_PHONE_NUMBER`.
