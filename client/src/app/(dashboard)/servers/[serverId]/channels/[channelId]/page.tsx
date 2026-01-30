'use client';

import { use } from 'react';
import ServerHeader from "@/components/server/server-header";
import MessageInput from "@/components/chat/message-input";
import MessageList from "@/components/chat/message-list";
import { Status } from "@/types/models/status";

export default function ChannelPage({
    params
}: {
    params: Promise<{ serverId: string; channelId: string }>
}) {
    const { serverId, channelId } = use(params);

    const channelsData: { [key: number]: { id: number; name: string } } = {
        1: { id: 1, name: 'Général' },
        2: { id: 2, name: 'information' },
        3: { id: 3, name: 'Invites' },
        4: { id: 4, name: 'general' },
        5: { id: 5, name: 'announcements' },
        6: { id: 6, name: 'dev-chat' },
        7: { id: 7, name: 'code-review' },
    };

    const currentChannel = channelsData[parseInt(channelId)] || { id: 1, name: 'général' };

    const messagesData: { [key: number]: any[] } = {
        1: [
            {
                id: 1,
                channel_id: 1,
                user_id: 1,
                content: 'Bienvenue dans #général !',
                created_at: new Date(),
                user: {
                    id: 1,
                    username: 'Alice',
                    email: 'alice@gmail.com',
                    created_at: new Date(),
                    status: Status.ONLINE,
                },
            },
            {
                id: 2,
                channel_id: 1,
                user_id: 2,
                content: 'Salut tout le monde !',
                created_at: new Date(),
                user: {
                    id: 2,
                    username: 'Bob',
                    email: 'bob@gmail.com',
                    created_at: new Date(),
                    status: Status.ONLINE,
                },
            },
        ],
        2: [
            {
                id: 3,
                channel_id: 2,
                user_id: 1,
                content: 'Ceci est le channel information',
                created_at: new Date(),
                user: {
                    id: 1,
                    username: 'Alice',
                    email: 'alice@gmail.com',
                    created_at: new Date(),
                    status: Status.ONLINE,
                },
            },
        ],
        3: [
            {
                id: 4,
                channel_id: 3,
                user_id: 2,
                content: 'Channel invites ici',
                created_at: new Date(),
                user: {
                    id: 2,
                    username: 'Bob',
                    email: 'bob@gmail.com',
                    created_at: new Date(),
                    status: Status.ONLINE,
                },
            },
        ],
        4: [
            {
                id: 5,
                channel_id: 4,
                user_id: 1,
                content: 'Welcome to the general channel!',
                created_at: new Date(),
                user: {
                    id: 1,
                    username: 'Alice',
                    email: 'alice@gmail.com',
                    created_at: new Date(),
                    status: Status.ONLINE,
                },
            },
        ],
        5: [
            {
                id: 6,
                channel_id: 5,
                user_id: 1,
                content: 'Important announcements here',
                created_at: new Date(),
                user: {
                    id: 1,
                    username: 'Alice',
                    email: 'alice@gmail.com',
                    created_at: new Date(),
                    status: Status.ONLINE,
                },
            },
        ],
        6: [
            {
                id: 7,
                channel_id: 6,
                user_id: 2,
                content: 'Let\'s discuss development',
                created_at: new Date(),
                user: {
                    id: 2,
                    username: 'Bob',
                    email: 'bob@gmail.com',
                    created_at: new Date(),
                    status: Status.ONLINE,
                },
            },
        ],
        7: [
            {
                id: 8,
                channel_id: 7,
                user_id: 1,
                content: 'Code review requests go here',
                created_at: new Date(),
                user: {
                    id: 1,
                    username: 'Alice',
                    email: 'alice@gmail.com',
                    created_at: new Date(),
                    status: Status.ONLINE,
                },
            },
        ],
    };

    // Récupère les messages du channel actuel
    const messageList = messagesData[parseInt(channelId)] || [];

    return (
        <main className="flex-1 flex flex-col min-w-0">
            <ServerHeader channelName={currentChannel.name} />

            <div className="flex-1 overflow-hidden">
                <MessageList messages={messageList} />
            </div>

            <MessageInput channelName={currentChannel.name} />
        </main>
    );
}