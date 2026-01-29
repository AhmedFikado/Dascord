"use client";

import Sidebar from "@/components/layout/sidebar";
import ServerSidebar from "@/components/server/server-sidebar";
import MemberSidebar from "@/components/server/member-sidebar";
import ServerHeader from "@/components/server/server-header";
import MessageInput from "@/components/chat/message-input";
import MessageList from "@/components/chat/message-list";
import { Channel } from "@/types/models/channel";
import { use } from "react";
import { Status } from "@/types/models/status";
import UserCard from '@/components/shared/user-card';
import { User } from "@/types/models/user";
import { Button } from "@/components/ui/button";
import { Settings } from 'lucide-react';

export default function Home() {
  const channelsList: Channel[] = [
    { id: 1, server_id: 1, name: 'Général', created_at: new Date() },
    { id: 2, server_id: 2, name: 'information', created_at: new Date() },
    { id: 3, server_id: 3, name: 'Invites', created_at: new Date() },
  ];

  const user: User = {
    id: 1,
    username: 'Alice',
    email: 'alice@gmail.com',
    created_at: new Date(),
    status: Status.ONLINE,
  };

  const messageList = [
    {
      id: 1,
      channel_id: 1,
      user_id: 1,
      content: 'Bonjour tout le monde !',
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
      content: 'Salut !',
      created_at: new Date(),
      user: {
        id: 2,
        username: 'Bob',
        email: 'bob@gmail.com',
        created_at: new Date(),
        status: Status.ONLINE,

      },
    },
    {
      id: 3,
      channel_id: 1,
      user_id: 1,
      content: 'Comment ça va ?',
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
      id: 4,
      channel_id: 1,
      user_id: 2,
      content: 'Bien, merci !',
      created_at: new Date(),
      user: {
        id: 2,
        username: 'Bob',
        email: 'bob@gmail.com',
        created_at: new Date(),
        status: Status.ONLINE,
      },
    },
  ];


  return (
    <div className="flex h-screen w-screen overflow-hidden">

      <Sidebar />
      <ServerSidebar serverId={1} />

      <div className="flex fixed bottom-2 rounded-2xl pl-3 py-1 left-3 gap-3 bg-gray-400 w-72 items-center justify-between">
        <div className="flex items-center gap-3">
          <UserCard user={{ username: 'Alice' }} />
          <div className="flex-1 min-w-0">
            <span className="font-semibold text-white hover:underline cursor-pointer">
              {user.username}
            </span>
          </div>
        </div>
        <Button
          variant="noBackground"
          width="50px"
          height="50px"
        >
          <Settings color="#adadad" size={24} /></Button>
      </div>


      <main className="flex-1 flex flex-col min-w-0">
        <ServerHeader channelName={channelsList[0].name} />

        <div className="flex-1 overflow-hidden">
          <MessageList messages={messageList} />
        </div>

        <MessageInput channelName={channelsList[0].name} />
      </main>

      <MemberSidebar />
    </div>
  );
}