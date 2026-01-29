db = db.getSiblingDB('rtc_db');

// Créer la collection messages (équivalent table en SQL)
db.createCollection('messages');

// Index pour récupérer les messages d'un channel, triés par date (1 = croissant, -1 = décroissant)
db.messages.createIndex({ channel_id: 1, created_at: -1 }, { name: 'idx_channel_messages' });

// Index pour trouver les messages d'un utilisateur (pour suppression en cascade)
db.messages.createIndex({ user_id: 1 }, { name: 'idx_user_messages' });

print("MongoDB: Collection 'messages' créée avec indexes");
