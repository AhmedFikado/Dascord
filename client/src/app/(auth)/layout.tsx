import type { Metadata } from 'next'
import './globals.css'

export const metadata: Metadata = {
    title: 'Discord Clone',
    description: 'A Discord-like application',
}

export default function RootLayout({
    children,
}: {
    children: React.ReactNode
}) {
    return (
        <html lang="en">
            <body>{children}</body>
        </html>
    )
}