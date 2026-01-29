import { User, Status } from "../../types/models/user";

export default function MemberItem({ user }: { user: User }) {

    const getStatusIndicator = (status: Status) => {
        const baseClass = "absolute -bottom-0.5 -right-0.5 w-3 h-3 rounded-full border-2 border-backgroundSide";

        switch (status) {
            case Status.ONLINE:
                return <div className={baseClass}><div className="w-full h-full rounded-full bg-green"></div></div>;

            case Status.OFFLINE:
                return <div className={baseClass}><div className="w-full h-full rounded-full bg-gray-50"></div></div>;

            default:
                return <div className={baseClass}><div className="w-full h-full rounded-full bg-gray-50"></div></div>;
        }
    };

    return (
        <div className="flex items-center gap-3 px-2 py-1.5 mx-2 rounded hover:bg-hoverSide cursor-pointer group transition-colors">
            <div className="relative flex-shrink-0">
                <div className="w-8 h-8 rounded-full bg-blurple flex items-center justify-center text-white font-semibold text-sm">
                    {user.username.charAt(0).toUpperCase()}
                </div>
                {getStatusIndicator(user.status)}
            </div>
            <span className="text-sm font-medium text-gray-light group-hover:text-white transition-colors truncate">
                {user.username}
            </span>
        </div>
    );
}