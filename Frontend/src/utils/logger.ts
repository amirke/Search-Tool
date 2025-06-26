export function logWithTime(msg: string) {
 //   const time = new Date().toLocaleTimeString();
    const now = new Date();
    const time = now.toISOString().split('T')[1].replace('Z', '');
// Example: "14:23:56.123"

    console.log(`[${time}] ${msg}`);
  }
  