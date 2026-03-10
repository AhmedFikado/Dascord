# Dascord - RCT Application

Bienvenue sur Dascord ! une application inspiré de discord.

## 🏗️ Architecture

- **Backend**: Rust (Axum) avec architecture hexagonale
- **Frontend**: Next.js 16 avec React 19 et TypeScript
- **Bases de données**: PostgreSQL (données relationnelles) + MongoDB (messages)
- **Communication temps réel**: WebSocket

## 📋 Prérequis

- Docker & Docker Compose
- Node.js 20+ (pour développement local)
- Rust (nightly) (pour développement local)

## 🚀 Démarrage rapide

### Avec Docker Compose

```bash
# Cloner le projet
git clone <repository-url>
cd T-JSF-600-BDX

# Configurer les variables d'environnement
cp .env.example .env

# Lancer tous les services
docker-compose up -d --build
```

L'application sera accessible sur:
- Frontend: http://localhost:3000
- Backend API: http://localhost:8080
- WebSocket: ws://localhost:8080/ws

## 🔧 Configuration

### Variables d'environnement

Créez un fichier `.env` à la racine du projet:

```env
# PostgreSQL
POSTGRES_USER=user
POSTGRES_PASSWORD=user_password
POSTGRES_DB=postgres_db

# MongoDB
MONGO_USER=user
MONGO_PASSWORD=user_password
MONGO_DB=mongo_db

# JWT
JWT_SECRET=your_jwt_secret_key
JWT_EXPIRATION=3600

# Si vous voulez qu'un autre pc se connecte en étant sur le même wifi
HOST_IP (mettre votre IPv4 ici)
API_PORT
FRONTEND_PORT
```

## 📁 Structure du projet

```
T-JSF-600-BDX/
├── server/          # Backend Rust
│   ├── src/
│   │   ├── api/           # Handlers et routes HTTP
│   │   ├── application/   # Use cases et DTOs
│   │   ├── domain/        # Entités et value objects
│   │   ├── infrastructure/# Repositories, DB, WebSocket
│   │   └── utils/         # Utilitaires et gestion d'erreurs
│   ├── Cargo.toml
│   └── Dockerfile
├── client/          # Frontend Next.js
│   ├── src/
│   ├── package.json
│   └── Dockerfile
└── docker-compose.yml
```

## Fonctionnalités

### Authentification
- ✅ Inscription / Connexion
- ✅ JWT tokens
- ✅ Gestion des sessions
- ✅ Statuts utilisateur (Online/Offline)

### Serveurs
- ✅ Créer un serveur
- ✅ Rejoindre via code d'invitation
- ✅ Gestion des rôles (Owner, Admin, Member)
- ✅ Quitter un serveur

### Channels
- ✅ Créer des channels
- ✅ Modifier/Supprimer des channels
- ✅ Permissions basées sur les rôles

### Messages
- ✅ Envoi de messages en temps réel
- ✅ Historique des messages
- ✅ Modifier/Supprimer ses messages
- ✅ Indicateur de frappe
- ✅ Persistance des messages

### WebSocket
- ✅ Connexion authentifiée
- ✅ Rooms par channel
- ✅ Broadcast des messages
- ✅ Notifications de statut

## API Endpoints

### Authentification
- `POST /auth/signup` - Créer un compte
- `POST /auth/login` - Se connecter
- `POST /auth/logout` - Se déconnecter
- `GET /auth/me` - Informations utilisateur

### Utilisateurs
- `GET /users/me` - Profil utilisateur
- `PUT /users/me/status` - Mettre à jour le statut

### Serveurs
- `POST /servers` - Créer un serveur
- `GET /servers` - Liste des serveurs
- `GET /servers/:id` - Détails d'un serveur
- `PUT /servers/:id` - Modifier un serveur
- `DELETE /servers/:id` - Supprimer un serveur
- `POST /servers/join` - Rejoindre un serveur
- `DELETE /servers/:id/leave` - Quitter un serveur
- `GET /servers/:id/members` - Liste des membres
- `PUT /servers/:id/members/:userId` - Modifier le rôle
- `GET /servers/:id/channels` - Liste des channels
- `POST /servers/:id/channels` - Créer un channel

### Channels
- `GET /channels/:id` - Détails d'un channel
- `PUT /channels/:id` - Modifier un channel
- `DELETE /channels/:id` - Supprimer un channel

### Messages
- `POST /channels/:id/messages` - Envoyer un message
- `GET /channels/:id/messages` - Historique des messages
- `PUT /messages/:id` - Modifier un message
- `DELETE /messages/:id` - Supprimer un message

### WebSocket
- `WS /ws?token=<jwt_token>` - Connexion WebSocket

## 🧪 Tests

Le backend inclut des tests unitaires complets:

```bash
# Installer llvm et ses dépendances sur la machine
cargo install cargo-llvm-cov
rustup component add llvm-tools-preview

# Générer la couverture de code sans les mocks
cargo llvm-cov --html --ignore-filename-regex "mocks|mock_"
```

Allez ensuite dans /target/llmv-cov/html et ouvrez le index.html pour accéder au coverage

## 🐳 Docker

### Services disponibles

- `server`: Backend Rust (port 8080)
- `client`: Frontend Next.js (port 3000)
- `postgres`: Base de données PostgreSQL (port 5433)
- `mongodb`: Base de données MongoDB (port 27017)

### Commandes utiles

```bash
# Démarrer tous les services
docker-compose up -d

# Voir les logs
docker-compose logs -f

# Arrêter les services
docker-compose down

# Rebuild après modifications
docker-compose up -d --build
```

## 🔐 Sécurité

- Mots de passe hashés avec Argon2
- Authentification JWT
- Validation des entrées avec Validator
- Protection CORS configurée
- WebSocket authentifié

## 📚 Technologies utilisées

### Backend
- **Axum** - Framework web
- **SQLx** - Client PostgreSQL
- **MongoDB** - Driver MongoDB
- **Tokio** - Runtime async
- **Argon2** - Hashing de mots de passe
- **jsonwebtoken** - Gestion JWT
- **Serde** - Sérialisation/Désérialisation

### Frontend
- **Next.js 16** - Framework React
- **React 19** - Bibliothèque UI
- **TypeScript** - Typage statique
- **Tailwind CSS** - Styling
- **Material-UI** - Composants UI
- **Zustand** - State management
- **Axios** - Client HTTP
