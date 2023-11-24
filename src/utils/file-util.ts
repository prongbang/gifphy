export let videoExtensions = ['mp4', 'avi', 'mov', 'mkv', 'wmv', 'flv', 'webm', 'mpeg', '3gp', 'mpg', 'm4v', 'rmvb'];

export function getFilename(path: string): string {
    return path.split("/").pop() ?? '';
}

export function getName(path: string): string {
    const filenameWithExtension = getFilename(path);
    const filename = filenameWithExtension.split('.').shift();
    return filename || '';
}