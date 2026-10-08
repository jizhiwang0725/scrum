import { useId, type ReactNode } from 'react'
import { Search } from 'lucide-react'
import { cn } from '@/lib/utils'

export interface EmptyStateProps {
  title?: string
  message?: ReactNode
  icon?: ReactNode
  action?: ReactNode
  className?: string
}

export function EmptyState({
  title = 'No results found',
  message = 'Try a different search or adjust your filters.',
  icon = <Search className="size-8" />,
  action,
  className,
}: EmptyStateProps) {
  const titleId = useId()

  return (
    <section
      aria-labelledby={titleId}
      className={cn(
        'flex flex-col items-center justify-center gap-4 px-6 py-12 text-center font-sans',
        className,
      )}
    >
      <div
        aria-hidden="true"
        className="flex size-16 items-center justify-center rounded-full bg-primary/10 text-primary"
      >
        {icon}
      </div>
      <h2 id={titleId} className="font-heading text-2xl font-semibold text-foreground">
        {title}
      </h2>
      <div className="max-w-md text-base break-words text-muted-foreground">
        {message}
      </div>
      {action && <div className="mt-2 flex flex-wrap justify-center gap-3">{action}</div>}
    </section>
  )
}
