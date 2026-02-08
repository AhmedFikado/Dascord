db = db.getSiblingDB('rtc_db');

// Créer la collection messages (équivalent table en SQL)
db.createCollection('messages');

// Index pour récupérer les messages d'un channel, triés par date (1 = croissant, -1 = décroissant)
db.messages.createIndex({ channel_id: 1, created_at: -1 }, { name: 'idx_channel_messages' });

// Index pour trouver les messages d'un utilisateur (pour suppression en cascade)
db.messages.createIndex({ user_id: 1 }, { name: 'idx_user_messages' });

print("MongoDB: Collection 'messages' créée avec indexes");

// Données de démonstration pour tests
db.messages.insertMany([
    {
        channel_id: "20000000-0000-0000-0000-000000000001",
        user_id: "00000000-0000-0000-0000-000000000001",
        username: "alice",
        content: "Hey everyone! Welcome to Gaming Squad! 🎮",
        created_at: new Date("2026-01-29T10:00:00Z")
    },
    {
        channel_id: "20000000-0000-0000-0000-000000000001",
        user_id: "00000000-0000-0000-0000-000000000002",
        username: "bob",
        content: "Thanks Alice! Happy to be here",
        created_at: new Date("2026-01-29T10:05:00Z")
    },
    {
        channel_id: "20000000-0000-0000-0000-000000000002",
        user_id: "00000000-0000-0000-0000-000000000003",
        username: "charlie",
        content: "Anyone up for a game tonight?",
        created_at: new Date("2026-01-29T11:00:00Z")
    },
    {
        channel_id: "20000000-0000-0000-0000-000000000003",
        user_id: "00000000-0000-0000-0000-000000000002",
        username: "bob",
        content: "Welcome to the Dev Team channel",
        created_at: new Date("2026-01-29T09:00:00Z")
    },
    {
        channel_id: "20000000-0000-0000-0000-000000000003",
        user_id: "00000000-0000-0000-0000-000000000004",
        username: "diana",
        content: "Ready to code! What are we working on?",
        created_at: new Date("2026-01-29T09:30:00Z")
    },
    {
        channel_id: "20000000-0000-0000-0000-000000000004",
        user_id: "00000000-0000-0000-0000-000000000001",
        username: "alice",
        content: "Let's discuss the new architecture",
        created_at: new Date("2026-01-29T12:00:00Z")
    }
]);

print("MongoDB: Données de démonstration insérées");
