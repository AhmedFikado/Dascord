export interface PrivateChannel {
  id: string;
  user1: string;
  user2: string;
  created_at: Date;
  updated_at: Date;
}

export interface PrivateChannelWithUser extends PrivateChannel {
  recipient_user?: {
    id: string;
    username: string;
    email: string;
    status: string;
    avatar_id?: string;
  };
}