/**
 * A path that, when it does not fit, gives up its folders before its file name:
 * "/home/sam/Downl…/invoice.pdf.desktop". The full path is the tooltip.
 */
export function PathText({ path, className = "" }: { path: string; className?: string }) {
  const cut = path.lastIndexOf("/", path.length - 2);
  const folder = cut >= 0 ? path.slice(0, cut + 1) : "";
  const name = cut >= 0 ? path.slice(cut + 1) : path;
  return (
    <span title={path} className={`flex min-w-0 font-mono ${className}`}>
      <span className="min-w-0 truncate">{folder}</span>
      <span className="max-w-full shrink-0 truncate">{name}</span>
    </span>
  );
}
