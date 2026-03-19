'use client';

import { useRouter, usePathname } from 'next/navigation';

export default function DirectMessagesButton() {
  const router = useRouter();
  const pathname = usePathname();
  const isActive = pathname?.startsWith('/dms');

  const handleClick = () => {
    router.push('/dms');
  };

  return (
    <button
      onClick={handleClick}
      className={`
        w-12 h-12 min-w-12 min-h-12 flex-shrink-0
        flex items-center justify-center 
        text-white font-bold text-xl cursor-pointer 
        transition-all duration-200 mb-2 rounded-[16px]
        ${isActive
          ? 'bg-blurple rounded-[16px]'
          : 'bg-gray-300 rounded-[16px] hover:bg-blurple hover:rounded-[16px]'
        }
      `}
      title="Messages privés"
    >
      💬
    </button>
  );
}
