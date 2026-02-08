CREATE TYPE role_type AS ENUM ('OWNER', 'ADMIN', 'MEMBER');

CREATE TYPE user_status AS ENUM ('ONLINE', 'OFFLINE');


CREATE TABLE users (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    username VARCHAR(50) UNIQUE NOT NULL,
    email VARCHAR(255) UNIQUE NOT NULL,
    password_hash VARCHAR(255) NOT NULL,
    status user_status DEFAULT 'OFFLINE',
    created_at TIMESTAMPTZ DEFAULT NOW()
);

CREATE TABLE servers (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    name VARCHAR(100) NOT NULL,
    owner_id UUID NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    invitation_code VARCHAR(30) UNIQUE NOT NULL,
    created_at TIMESTAMPTZ DEFAULT NOW()
);

CREATE TABLE server_members (
    server_id UUID REFERENCES servers(id) ON DELETE CASCADE,
    user_id UUID REFERENCES users(id) ON DELETE CASCADE,
    role role_type NOT NULL DEFAULT 'MEMBER',
    joined_at TIMESTAMPTZ DEFAULT NOW(),
    PRIMARY KEY (server_id, user_id)
);

CREATE TABLE channels (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    server_id UUID NOT NULL REFERENCES servers(id) ON DELETE CASCADE,
    name VARCHAR(100) NOT NULL,
    created_at TIMESTAMPTZ DEFAULT NOW()
);

-- Recherche rapide des serveurs d'un utilisateur
CREATE INDEX idx_server_members_user_id ON server_members(user_id);

-- Recherche rapide des channels d'un serveur
CREATE INDEX idx_channels_server_id ON channels(server_id);

-- Utilisateurs de test
INSERT INTO users (id, username, email, password_hash, status) VALUES
    ('00000000-0000-0000-0000-000000000001', 'alice', 'alice@example.com', '$argon2i$v=19$m=16,t=2,p=1$UWJrblNFQ2hweGVaeEZkeA$JVqT/cAE59JGhfLl/rTMPQ', 'ONLINE'),
    ('00000000-0000-0000-0000-000000000002', 'bob', 'bob@example.com', '$argon2i$v=19$m=16,t=2,p=1$UWJrblNFQ2hweGVaeEZkeA$JVqT/cAE59JGhfLl/rTMPQ', 'ONLINE'),
    ('00000000-0000-0000-0000-000000000003', 'charlie', 'charlie@example.com', '$argon2i$v=19$m=16,t=2,p=1$UWJrblNFQ2hweGVaeEZkeA$JVqT/cAE59JGhfLl/rTMPQ', 'OFFLINE'),
    ('00000000-0000-0000-0000-000000000004', 'diana', 'diana@example.com', '$argon2i$v=19$m=16,t=2,p=1$UWJrblNFQ2hweGVaeEZkeA$JVqT/cAE59JGhfLl/rTMPQ', 'ONLINE');

-- Serveurs de test
INSERT INTO servers (id, name, owner_id, invitation_code) VALUES
    ('10000000-0000-0000-0000-000000000001', 'Gaming Squad', '00000000-0000-0000-0000-000000000001', 'GAME2025'),
    ('10000000-0000-0000-0000-000000000002', 'Dev Team', '00000000-0000-0000-0000-000000000002', 'DEV2025');

-- Membres des serveurs
INSERT INTO server_members (server_id, user_id, role) VALUES
    ('10000000-0000-0000-0000-000000000001', '00000000-0000-0000-0000-000000000001', 'OWNER'),
    ('10000000-0000-0000-0000-000000000001', '00000000-0000-0000-0000-000000000002', 'ADMIN'),
    ('10000000-0000-0000-0000-000000000001', '00000000-0000-0000-0000-000000000003', 'MEMBER'),
    ('10000000-0000-0000-0000-000000000002', '00000000-0000-0000-0000-000000000002', 'OWNER'),
    ('10000000-0000-0000-0000-000000000002', '00000000-0000-0000-0000-000000000001', 'MEMBER'),
    ('10000000-0000-0000-0000-000000000002', '00000000-0000-0000-0000-000000000004', 'MEMBER');

-- Channels
INSERT INTO channels (id, server_id, name) VALUES
    ('20000000-0000-0000-0000-000000000001', '10000000-0000-0000-0000-000000000001', 'general'),
    ('20000000-0000-0000-0000-000000000002', '10000000-0000-0000-0000-000000000001', 'gaming'),
    ('20000000-0000-0000-0000-000000000003', '10000000-0000-0000-0000-000000000002', 'general'),
    ('20000000-0000-0000-0000-000000000004', '10000000-0000-0000-0000-000000000002', 'tech-discussion');
