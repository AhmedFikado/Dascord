'use client';

import React from 'react';
import { Loading } from '../shared/loading-spinner';

interface ButtonProps extends React.ButtonHTMLAttributes<HTMLButtonElement> {
    width?: string;
    height?: string;
    variant?: 'primary' | 'secondary' | 'danger' | 'outline' | 'noBackground';
    isLoading?: boolean;
    children: React.ReactNode;
}

export const Button: React.FC<ButtonProps> = ({
    width = 'auto',
    height = '40px',
    variant = 'primary',
    isLoading = false,
    children,
    className = '',
    style,
    disabled,
    ...props
}) => {

    const variants = {
        primary: 'bg-blurple hover:bg-blurple/80 text-white',
        secondary: 'bg-gray-200 hover:bg-gray-100 text-white',
        danger: 'bg-red hover:bg-red/80 text-white',
        outline: 'bg-transparent border border-blurple text-blurple hover:bg-blurple/10',
        noBackground: 'bg-transparent hover:bg-gray-200',
    };

    return (
        <button
            style={{ width, height, ...style }}
            className={`
                rounded-lg px-4 font-medium transition-colors duration-200 
                flex items-center justify-center gap-2
                disabled:opacity-50 disabled:cursor-not-allowed
                ${variants[variant]}
                ${className}
            `}
            disabled={isLoading || disabled}
            {...props}
        >
            {isLoading ? (
                <Loading size="sm" />
            ) : (
                children
            )}
        </button>
    );
};