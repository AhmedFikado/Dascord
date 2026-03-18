import { Status } from "@/types/models/status";

interface UserCardProps {
    username?: string;
    size?: number;
    status?: Status;
}

export default function UserCard({ username, size = 40, status }: UserCardProps) {

    const getInitials = (username?: string) => {
        return username?.charAt(0).toUpperCase() || '?';
    };

    const getColorFromUsername = (username?: string) => {
        if (!username) return '#5865F2';

        const colors = [
            '#5865F2', // bleue
            '#47c16e', // vert
            '#ED4245', // rouge
            '#e9d253', // jaune
            '#e71d89', // rose
            '#c3511c', // orange
            '#01b9da', // cyan
            '#9B59B6', // violet
            '#870606', // rouge foncé
            '#1ABC9C', // turquoise
        ];

        if (username === 'Système') return '#3e3e3f';

        let hash = 0;
        for (let i = 0; i < username.length; i++) {
            hash = username.charCodeAt(i) + ((hash * 31) - hash);
        }

        return colors[Math.abs(hash) % colors.length];
    };

    const getStatusIndicator = (status: Status) => {
        const baseClass = "absolute -bottom-0.5 -right-0.5 w-3 h-3 rounded-full border-2 border-backgroundSide";

        switch (status) {
            case Status.ONLINE:
                return <div className={baseClass}><div className="w-full h-full rounded-full bg-green"></div></div>;
            case Status.OFFLINE:
                return <div className={baseClass}><div className="w-full h-full rounded-full bg-gray-50"></div></div>;
            case Status.INACTIVE:
                return <div className={baseClass}><div className="w-full h-full rounded-full bg-orange-400"></div></div>;
            case Status.DONOTDISTURB:
                return <div className={baseClass}><div className="w-full h-full rounded-full bg-red-400"></div></div>;
        }
    };

    return (
        <div className="relative flex-shrink-0">
            <div
                className="rounded-full flex items-center justify-center text-white font-semibold"
                style={{
                    backgroundColor: getColorFromUsername(username),
                    width: `${size}px`,
                    height: `${size}px`,
                    fontSize: `${size * 0.4}px`
                }}
            >
                {getInitials(username)}
            </div>
            {status && getStatusIndicator(status)}
        </div>

    );
}