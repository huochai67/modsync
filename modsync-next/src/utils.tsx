export const formatBytes = (bytes?: number, need_sign: boolean = false) => {
  if (!bytes || bytes === 0) return "0 B";
  const k = 1024;
  const sizes = ["B", "KB", "MB", "GB"];
  const newbytes = bytes < 0 ? -bytes : bytes;
  const i = Math.floor(Math.log(newbytes) / Math.log(k));
  const sign = bytes < 0 ? "" : "+";
  return `${need_sign ? sign : ""}${parseFloat((bytes / Math.pow(k, i)).toFixed(2))} ${sizes[i]}`;
};
