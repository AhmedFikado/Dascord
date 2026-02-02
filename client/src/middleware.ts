import { NextResponse } from 'next/server'
import type { NextRequest } from 'next/server'

const protectedRoutes = ['/servers', '/dashboard'];
const publicRoutes = ['/login', '/signup', '/'];

export function middleware(request: NextRequest) {
    const { pathname } = request.nextUrl;
    const isProtectedRoute = protectedRoutes.some(route => pathname.startsWith(route));

    const token = request.cookies.get('auth_token')?.value;

    if (isProtectedRoute && !token) {
        const loginUrl = new URL('/login', request.url);
        return NextResponse.redirect(loginUrl);
    }

    if ((pathname === '/login' || pathname === '/signup') && token) {
        const serversUrl = new URL('/servers', request.url);
        return NextResponse.redirect(serversUrl);
    }

    return NextResponse.next()
}

export const config = {
    matcher: [
        '/((?!_next/static|_next/image|favicon.ico).*)',
    ],
}