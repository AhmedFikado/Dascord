'use client';

import { PrivateChannelWithUser } from '../../types/models/privateChannels';

interface DMHeaderProps {
  conversation: PrivateChannelWithUser;
}

export default function DMHeader({ conversation }: DMHeaderProps) {
  const recipientUser = conversation.recipient_user;

  return (
    <div className="flex items-center justify-between">
      <div className="flex items-center gap-3 flex-1">
        {/* Avatar */}
        <div className="w-10 h-10 rounded-full flex items-center justify-center bg-gray-400 text-white font-bold">
          {recipientUser?.username?.[0]?.toUpperCase() || '?'}
        </div>
        
        {/* User info */}
        <div className="flex-1 min-w-0">
          <h2 className="font-semibold text-white truncate">
            {recipientUser?.username || 'Unknown User'}
          </h2>
          <p className="text-xs text-gray-light capitalize">
            {recipientUser?.status || 'offline'}
          </p>
        </div>
      </div>
    </div>
  );
}
