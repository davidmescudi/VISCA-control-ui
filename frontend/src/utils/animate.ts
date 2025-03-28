export function getAnimationClass (success: boolean): string {
    return success ? 'animate-blinkGreen !ring-0' : 'animate-blinkRed !ring-0';
}