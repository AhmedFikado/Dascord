import { Message } from "@/types/models/message";
import { User, Status } from "@/types/models/user";

const userAlice: User = {
    id: 1,
    username: 'Alice',
    email: 'alice@gmail.com',
    created_at: new Date(),
    status: Status.ONLINE,
};

const userBob: User = {
    id: 2,
    username: 'Bob',
    email: 'bob@gmail.com',
    created_at: new Date(),
    status: Status.ONLINE,
};

const mockMessages: Record<number, Message[]> = {
    1: [ // Channel Général (Serveur 1)
        {
            id: 1,
            channel_id: 1,
            user_id: 1,
            content: 'Bienvenue dans #général !',
            created_at: new Date(),
            user: userAlice,
        },
        {
            id: 2,
            channel_id: 1,
            user_id: 2,
            content: 'Salut tout le monde !',
            created_at: new Date(),
            user: userBob,
        },
        {
            id: 3,
            channel_id: 1,
            user_id: 2,
            content: 'Salut tout le monde !',
            created_at: new Date(),
            user: userBob,
        },
        {
            id: 4,
            channel_id: 1,
            user_id: 2,
            content: 'Salut tout le monde !',
            created_at: new Date(),
            user: userBob,
        },
        {
            id: 5,
            channel_id: 1,
            user_id: 2,
            content: 'Salut tout le monde !',
            created_at: new Date(),
            user: userBob,
        },
        {
            id: 6,
            channel_id: 1,
            user_id: 2,
            content: 'Salut tout le monde !',
            created_at: new Date(),
            user: userBob,
        },
        {
            id: 7,
            channel_id: 1,
            user_id: 2,
            content: 'Salut tout le monde !',
            created_at: new Date(),
            user: userBob,
        },
        {
            id: 8,
            channel_id: 1,
            user_id: 2,
            content: 'Salut tout le monde !',
            created_at: new Date(),
            user: userBob,
        },
        {
            id: 9,
            channel_id: 1,
            user_id: 2,
            content: 'Salut tout le monde !',
            created_at: new Date(),
            user: userBob,
        },
        {
            id: 10,
            channel_id: 1,
            user_id: 2,
            content: 'Salut tout le monde !',
            created_at: new Date(),
            user: userBob,
        },
        {
            id: 11,
            channel_id: 1,
            user_id: 2,
            content: 'Salut tout le monde !',
            created_at: new Date(),
            user: userBob,
        },
        {
            id: 12,
            channel_id: 1,
            user_id: 2,
            content: 'Salut tout le monde !',
            created_at: new Date(),
            user: userBob,
        },
        {
            id: 13,
            channel_id: 1,
            user_id: 2,
            content: 'Salut tout le monde !',
            created_at: new Date(),
            user: userBob,
        },
        {
            id: 14,
            channel_id: 1,
            user_id: 2,
            content: 'Salut tout le monde !',
            created_at: new Date(),
            user: userBob,
        },
        {
            id: 15,
            channel_id: 1,
            user_id: 2,
            content: 'Salut tout le monde !',
            created_at: new Date(),
            user: userBob,
        },
        {
            id: 16,
            channel_id: 1,
            user_id: 2,
            content: 'Salut tout le monde !',
            created_at: new Date(),
            user: userBob,
        },
        {
            id: 17,
            channel_id: 1,
            user_id: 2,
            content: 'Salut tout le monde !',
            created_at: new Date(),
            user: userBob,
        },
        {
            id: 18,
            channel_id: 1,
            user_id: 2,
            content: 'Salut tout le monde !',
            created_at: new Date(),
            user: userBob,
        }, {
            id: 19,
            channel_id: 1,
            user_id: 2,
            content: 'Salut tout le monde !',
            created_at: new Date(),
            user: userBob,
        },
        {
            id: 20,
            channel_id: 1,
            user_id: 2,
            content: 'Salut tout le monde !',
            created_at: new Date(),
            user: userBob,
        },
        {
            id: 21,
            channel_id: 1,
            user_id: 2,
            content: 'Salut tout le monde !',
            created_at: new Date(),
            user: userBob,
        },
        {
            id: 22,
            channel_id: 1,
            user_id: 2,
            content: 'Salut tout le monde !',
            created_at: new Date(),
            user: userBob,
        },
        {
            id: 23,
            channel_id: 1,
            user_id: 2,
            content: 'Salut tout le monde !',
            created_at: new Date(),
            user: userBob,
        },

    ],
    2: [ // Channel Information (Serveur 1)
        {
            id: 3,
            channel_id: 2,
            user_id: 1,
            content: 'Ceci est le channel information',
            created_at: new Date(),
            user: userAlice,
        },
    ],
    3: [ // Channel Invites (Serveur 1)
        {
            id: 4,
            channel_id: 3,
            user_id: 2,
            content: 'Channel invites ici',
            created_at: new Date(),
            user: userBob,
        },
    ],
    4: [ // General (Serveur 2)
        {
            id: 5,
            channel_id: 4,
            user_id: 1,
            content: 'Welcome to the general channel!',
            created_at: new Date(),
            user: userAlice,
        },
    ],
    5: [ // Announcements (Serveur 2)
        {
            id: 6,
            channel_id: 5,
            user_id: 1,
            content: 'Important announcements here',
            created_at: new Date(),
            user: userAlice,
        },
    ],
    6: [ // Dev Chat (Serveur 3)
        {
            id: 7,
            channel_id: 6,
            user_id: 2,
            content: 'Let\'s discuss development',
            created_at: new Date(),
            user: userBob,
        },
    ],
    7: [ // Code Review (Serveur 3)
        {
            id: 8,
            channel_id: 7,
            user_id: 1,
            content: 'Code review requests go here',
            created_at: new Date(),
            user: userAlice,
        },
    ],
};


export const messagesApi = {

    // GET /channels/{id}/messages
    getMessageHistory: async (channelId: number): Promise<Message[]> => {
        return [...(mockMessages[channelId] || [])];
    },

    // POST /channels/{id}/messages 
    send: async (channelId: number, content: string): Promise<Message> => {
        const newMessage: Message = {
            id: Date.now(),
            channel_id: channelId,
            user_id: 1, //par là suite on récupe le state user pour mettre le bonne ID
            content: content,
            created_at: new Date(),
            user: userAlice //idem avec le state on donne le user
        };

        if (!mockMessages[channelId]) {
            mockMessages[channelId] = [];
        }

        mockMessages[channelId].push(newMessage);

        return newMessage;
    },

    // DELETE /messages/{id}
    delete: async (channelId: number, messageId: number): Promise<void> => {
        if (mockMessages[channelId]) {
            mockMessages[channelId] = mockMessages[channelId].filter(m => m.id !== messageId);
        }
    }
};