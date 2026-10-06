


export function getScoreClass(score: number) {
    if (score >= 0.8) {
        return ' bg-gradient-to-r from-pink-100 via-purple-100 to-cyan-100 text-purple-700 shadow-[0_0_12px_rgba(168,85,247,0.25)]'
    }
    if (score >= 0.6) {
        return 'border-amber-300/60 bg-gradient-to-r from-amber-50 via-yellow-100 to-amber-50 text-amber-700 shadow-[0_0_8px_rgba(245,158,11,0.18)]'
    }
    if (score >= 0.3) {
        return 'border-slate-300/60 bg-gradient-to-r from-slate-50 via-white to-slate-100 text-slate-600 shadow-[0_0_7px_rgba(148,163,184,0.2)]'
    }

    if (score >= 0.2) {
        return 'bg-success/10 border-success/40 text-success font-semibold shadow-sm'
    }

    if (score >= 0.1) {
        return 'bg-warning/10 border-warning/40 text-warning font-semibold'
    }

    return 'bg-base-200/50 border-base-300 text-base-content/50'
}