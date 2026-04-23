import { apiClient } from './client';

export interface UnreadChannelDto {
  channel_id: string;
  first_unread_message_id: string;
}

export interface UnreadStatusResponse {
  unread_channels: UnreadChannelDto[];
}

export async function markChannelAsRead(channelId: string): Promise<void> {
  await apiClient.post(`/channels/${channelId}/read`);
}

export async function getUnreadChannelsForServer(
  serverId: string
): Promise<UnreadStatusResponse> {
  return apiClient.get<UnreadStatusResponse>(`/servers/${serverId}/unread`);
}

export async function getUnreadPrivateChannels(): Promise<UnreadStatusResponse> {
  return apiClient.get<UnreadStatusResponse>(`/channels/private/unread`);
}
