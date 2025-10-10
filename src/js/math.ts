/** 
 * Maps a number from one range to another. 
 *  
 * @param value - The input number to map 
 * @param start1 - Lower bound of the input range 
 * @param stop1 - Upper bound of the input range 
 * @param start2 - Lower bound of the output range 
 * @param stop2 - Upper bound of the output range 
 * @param withinBounds - Optional flag to clamp the result within output bounds (default: false) 
 * @returns The mapped value in the target range 
 *  
 * @example 
 * // Basic mapping 
 * map(5, 0, 10, 0, 100); // → 50 
 *  
 * @example 
 * // With bounds clamping 
 * map(15, 0, 10, 0, 100, true); // → 100 
 *  
 * @example 
 * // Reverse mapping 
 * map(3, 0, 5, 100, 0); // → 40 
 */ 
export function map( 
    value: number, 
    start1: number, 
    stop1: number, 
    start2: number, 
    stop2: number, 
    withinBounds: boolean = false 
): number { 
    const output = ((value - start1) / (stop1 - start1)) * (stop2 - start2) + start2; 
     
    if (withinBounds) { 
        return start2 < stop2  
            ? Math.max(Math.min(output, stop2), start2) 
            : Math.max(Math.min(output, start2), stop2); 
    } 
     
    return output; 
} 


