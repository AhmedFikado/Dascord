'use client';

import React, { forwardRef } from 'react';

interface InputProps extends React.InputHTMLAttributes<HTMLInputElement> {
    label?: string;
    error?: string;
    helperText?: string;
    leftIcon?: React.ReactNode;
    rightIcon?: React.ReactNode;
    width?: string;
}

export const Input = forwardRef<HTMLInputElement, InputProps>(({
    label,
    error,
    helperText,
    leftIcon,
    rightIcon,
    width = '100%',
    className = '',
    id,
    disabled,
    ...props
}, ref) => {

    const generatedId = React.useId();
    const inputId = id || generatedId;

    return (
        <div style={{ width }} className={`flex flex-col gap-1.5 ${className}`}>

            {label && (
                <label
                    htmlFor={inputId}
                    className="text-sm font-medium text-white ml-[5px]"
                >
                    {label}
                </label>
            )}

            <div className="relative flex items-center">

                {leftIcon && (
                    <div className="absolute left-3 text-white pointer-events-none">
                        {leftIcon}
                    </div>
                )}

                <input
                    ref={ref}
                    id={inputId}
                    disabled={disabled}
                    suppressHydrationWarning={true}
                    className={`
                        w-full rounded-lg border bg-gray-400 px-3 py-2 text-sm transition-colors
                        placeholder:text-gray-50
                        focus:outline-none focus:ring-2

                        ${leftIcon ? 'pl-10' : ''}
                        ${rightIcon ? 'pr-10' : ''}

                        ${error
                        ? 'border-red focus:border-red focus:ring-red/50 text-red'
                        : 'border-gray-300 focus:border-blurple focus:ring-blurple/50 text-white'
                    }
                    `}
                    {...props}
                />

                {rightIcon && (
                    <div className="absolute right-3 text-gray-200 cursor-pointer hover:text-gray-100">
                        {rightIcon}
                    </div>
                )}
            </div>

            {error ? (
                <p className="text-xs text-red mt-0.4 ml-2">{error}</p>
            ) : helperText ? (
                <p className="text-xs text-gray-50 mt-0.4 ml-2">{helperText}</p>
            ) : null}
        </div>
    );
});

Input.displayName = 'Input';