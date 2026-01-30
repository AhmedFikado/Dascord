'use client';

import { use } from 'react';
import { MessageSquareDashed } from 'lucide-react';
import ServerHeader from "@/components/server/server-header";
import MessageInput from "@/components/chat/message-input";
import MessageList from "@/components/chat/message-list";
import { useChannels } from '@/app/lib/hooks/use-channels';
import { useMessages } from '@/app/lib/hooks/use-messages';

export default function ChannelPage({
    params
}: {
    params: Promise<{ serverId: string; channelId: string }>
}) {
    const { serverId, channelId } = use(params);

    const numericServerId = Number(serverId);
    const numericChannelId = Number(channelId);

    const { channels, isLoading: channelsLoading } = useChannels(numericServerId);

    const currentChannel = channels.find(c => c.id === numericChannelId);

    const { messages, sendMessage } = useMessages(currentChannel ? numericChannelId : 0);

    if (channelsLoading) {
        return (
            <div className="flex-1 flex flex-col items-center justify-center bg-background h-full">
                <div className="animate-spin rounded-full h-8 w-8 border-b-2 border-gray-900"></div>
                <p className="mt-4 text-gray-light text-sm">Chargement du serveur...</p>
            </div>
        );
    }

    if (!currentChannel) {
        return (
            <div className="flex-1 flex flex-col items-center justify-center bg-background h-full text-center px-4">
                <div className="bg-background p-4 rounded-full mb-2">
                    <MessageSquareDashed className="w-12 h-12 text-gray-light" />
                </div>
                <h3 className="text-xl font-semibold text-gray-light mb-2">
                    Aucun salon sélectionné
                </h3>
            </div>
        );
    }

    return (
        <main className="flex-1 flex flex-col min-w-0 bg-background h-full">
            <ServerHeader channelName={currentChannel.name} />

            <div className="flex-1 overflow-hidden flex flex-col">
                <MessageList messages={messages} />
            </div>

            <MessageInput
                channelName={currentChannel.name}
                onSendMessage={(content) => sendMessage(numericChannelId, content)}
            />
        </main>
    );
}