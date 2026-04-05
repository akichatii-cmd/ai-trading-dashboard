/** @type {import('tailwindcss').Config} */
module.exports = {
  content: {
    files: ["*.html", "./src/**/*.rs"],
  },
  theme: {
    extend: {
      colors: {
        void: '#0a0a0f',
        card: '#141419',
        hover: '#1a1a24',
        active: '#22222d',
        subtle: '#2a2a35',
        focus: '#3a3a4a',
        primary: '#ffffff',
        secondary: '#8a8a9a',
        muted: '#5a5a6a',
        success: '#00ff88',
        danger: '#ff3366',
        warning: '#ffcc00',
        info: '#00ccff',
        accent: '#ffdd2d',
      },
      fontFamily: {
        sans: ['Inter', '-apple-system', 'BlinkMacSystemFont', 'sans-serif'],
        mono: ['JetBrains Mono', 'monospace'],
      },
      fontSize: {
        '2xs': '10px',
      },
      spacing: {
        '18': '4.5rem',
        '22': '5.5rem',
      },
      borderRadius: {
        'sm': '4px',
        'md': '8px',
        'lg': '12px',
      },
      animation: {
        'pulse-warning': 'pulse-warning 2s ease-in-out infinite',
        'flash-danger': 'flash-danger 1s ease-in-out infinite',
        'pulse-border': 'pulse-border 1.5s ease-in-out infinite',
        'slide-up': 'slide-up 0.3s ease-out',
        'fade-in': 'fade-in 0.2s ease-out',
      },
      keyframes: {
        'pulse-warning': {
          '0%, 100%': { opacity: '1' },
          '50%': { opacity: '0.6' },
        },
        'flash-danger': {
          '0%, 100%': { background: 'var(--danger)' },
          '50%': { background: 'var(--danger-dim)' },
        },
        'pulse-border': {
          '0%, 100%': { borderColor: 'var(--success)' },
          '50%': { borderColor: 'transparent' },
        },
        'slide-up': {
          from: { transform: 'translateY(100%)', opacity: '0' },
          to: { transform: 'translateY(0)', opacity: '1' },
        },
        'fade-in': {
          from: { opacity: '0' },
          to: { opacity: '1' },
        },
      },
    },
  },
  plugins: [],
}
