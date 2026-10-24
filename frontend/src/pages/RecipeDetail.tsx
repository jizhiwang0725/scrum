import { useEffect, useState } from "react";
import {
  ArrowLeft,
  ChefHat,
  ExternalLink,
  ImageOff,
  ListOrdered,
  UtensilsCrossed,
} from "lucide-react";
import { Link, useParams } from "react-router-dom";
import { ErrorState } from "@/components/common/ErrorState";
import { Loading } from "@/components/common/Loading";
import { Button } from "@/components/ui/button";
import { cn } from "@/lib/utils";

export interface Recipe {
  id: string;
  title: string;
  description: string;
  ingredients: { name: string; amount: string }[];
  steps: { number: number; text: string; image_urls: string[] }[];
  tips: string;
  cover_image_urls: string[];
  author: string;
  author_url: string;
  url: string;
}

export type RecipeListLoader = (signal: AbortSignal) => Promise<unknown>;

type RequestState =
  | { status: "loading" }
  | { status: "success"; recipe: Recipe }
  | { status: "error"; title: string; message: string };

function isRecipe(value: unknown): value is Recipe {
  if (typeof value !== "object" || value === null) return false;
  const recipe = value as Record<string, unknown>;
  const strings = [
    "id",
    "title",
    "description",
    "tips",
    "author",
    "author_url",
    "url",
  ];

  return (
    strings.every((field) => typeof recipe[field] === "string") &&
    Array.isArray(recipe.cover_image_urls) &&
    recipe.cover_image_urls.every((url) => typeof url === "string") &&
    Array.isArray(recipe.ingredients) &&
    recipe.ingredients.every(
      (item) =>
        item !== null &&
        typeof item === "object" &&
        typeof item.name === "string" &&
        typeof item.amount === "string",
    ) &&
    Array.isArray(recipe.steps) &&
    recipe.steps.every(
      (step) =>
        step !== null &&
        typeof step === "object" &&
        Number.isInteger(step.number) &&
        step.number > 0 &&
        typeof step.text === "string" &&
        Array.isArray(step.image_urls) &&
        step.image_urls.every((url: unknown) => typeof url === "string"),
    )
  );
}

const loadApiRecipes: RecipeListLoader = async (signal) => {
  const baseUrl = (
    import.meta.env.VITE_API_BASE_URL ?? "http://localhost:3000"
  ).replace(/\/$/, "");
  let response: Response;
  let data: unknown;
  try {
    response = await fetch(`${baseUrl}/api/recipes`, {
      signal,
      headers: { Accept: "application/json" },
    });
    data = await response.json();
  } catch {
    throw new Error("请求失败，请重试。");
  }

  if (!response.ok) {
    if (
      typeof data === "object" &&
      data !== null &&
      "error" in data &&
      typeof data.error === "object" &&
      data.error !== null &&
      "message" in data.error &&
      typeof data.error.message === "string"
    ) {
      throw new Error(data.error.message || "请求失败，请重试。");
    }
    throw new Error("请求失败，请重试。");
  }

  return data;
};

function httpUrl(value: string | undefined) {
  if (!value) return undefined;
  try {
    const url = new URL(value);
    return ["https:", "http:"].includes(url.protocol) ? url.href : undefined;
  } catch {
    return undefined;
  }
}

function RecipeImage({
  src,
  alt,
  cover = false,
}: {
  src?: string;
  alt: string;
  cover?: boolean;
}) {
  const [failed, setFailed] = useState(false);
  const imageUrl = httpUrl(src);

  return (
    <div
      className={cn(
        "border-border bg-card overflow-hidden rounded-2xl border",
        cover && "aspect-[4/3]",
      )}
    >
      {imageUrl && !failed ? (
        <img
          src={imageUrl}
          alt={alt}
          loading={cover ? "eager" : "lazy"}
          referrerPolicy="no-referrer"
          onError={() => setFailed(true)}
          className={cn("w-full", cover ? "h-full object-cover" : "h-auto")}
        />
      ) : (
        <div
          role="img"
          aria-label={alt}
          className="text-muted-foreground flex h-full min-h-56 flex-col items-center justify-center gap-3 p-6"
        >
          <ImageOff aria-hidden="true" className="text-primary/60 size-10" />
          <p className="text-sm">
            {imageUrl ? "图片暂时无法显示" : "暂无菜谱图片"}
          </p>
        </div>
      )}
    </div>
  );
}

function RecipeContent({ recipe }: { recipe: Recipe }) {
  const authorUrl = httpUrl(recipe.author_url);
  const sourceUrl = httpUrl(recipe.url);
  const steps = [...recipe.steps].sort((a, b) => a.number - b.number);
  const linkClassName =
    "inline-flex items-center gap-2 rounded-md text-sm text-muted-foreground underline-offset-4 outline-none hover:text-primary hover:underline focus-visible:ring-3 focus-visible:ring-ring/50";

  return (
    <article className="space-y-12 md:space-y-16">
      <header className="grid items-center gap-8 lg:grid-cols-2 lg:gap-12">
        <RecipeImage
          src={recipe.cover_image_urls[0]}
          alt={recipe.title}
          cover
        />
        <div className="min-w-0 space-y-6">
          <p className="text-primary text-sm font-medium">用心做一道好菜</p>
          <h1 className="font-heading text-4xl leading-tight font-semibold break-words md:text-5xl">
            {recipe.title}
          </h1>
          {recipe.author && (
            <p className="text-muted-foreground text-sm">
              原作者：{" "}
              {authorUrl ? (
                <a
                  href={authorUrl}
                  target="_blank"
                  rel="noopener noreferrer"
                  className={linkClassName}
                >
                  {recipe.author}
                  <ExternalLink
                    aria-hidden="true"
                    className="size-3.5 shrink-0"
                  />
                </a>
              ) : (
                <span className="break-words">{recipe.author}</span>
              )}
            </p>
          )}
          {recipe.description.trim() && (
            <p className="text-muted-foreground text-base leading-8 break-words whitespace-pre-wrap">
              {recipe.description}
            </p>
          )}
          <div className="border-border text-muted-foreground flex flex-wrap gap-5 border-y py-5 text-sm">
            <span className="inline-flex items-center gap-2">
              <UtensilsCrossed
                aria-hidden="true"
                className="text-primary size-4"
              />
              {recipe.ingredients.length} 种食材
            </span>
            <span className="inline-flex items-center gap-2">
              <ListOrdered aria-hidden="true" className="text-primary size-4" />
              {steps.length} 个步骤
            </span>
          </div>
          <div className="flex flex-wrap items-center gap-5">
            <Button
              nativeButton={false}
              render={<a href="#recipe-steps" />}
              size="lg"
            >
              查看做法
            </Button>
            {sourceUrl && (
              <a
                href={sourceUrl}
                target="_blank"
                rel="noopener noreferrer"
                className={linkClassName}
              >
                查看原菜谱
                <ExternalLink aria-hidden="true" className="size-4" />
              </a>
            )}
          </div>
        </div>
      </header>

      <div className="grid items-start gap-10 lg:grid-cols-[minmax(0,1fr)_minmax(0,2fr)] lg:gap-14">
        <section
          aria-labelledby="recipe-ingredients"
          className="border-border bg-card text-card-foreground min-w-0 rounded-2xl border p-6 lg:sticky lg:top-8"
        >
          <h2
            id="recipe-ingredients"
            className="font-heading text-2xl font-semibold"
          >
            食材清单
          </h2>
          {recipe.ingredients.length > 0 ? (
            <dl className="divide-border mt-5 divide-y">
              {recipe.ingredients.map((ingredient, index) => (
                <div
                  key={`${ingredient.name}-${index}`}
                  className="grid grid-cols-2 gap-4 py-4 text-base leading-7"
                >
                  <dt className="font-medium break-words">{ingredient.name}</dt>
                  <dd className="text-muted-foreground text-right break-words whitespace-pre-wrap">
                    {ingredient.amount.trim()
                      ? ingredient.amount
                      : "用量未提供"}
                  </dd>
                </div>
              ))}
            </dl>
          ) : (
            <p className="text-muted-foreground mt-5 text-base">
              原菜谱未提供食材信息。
            </p>
          )}
        </section>

        <div className="min-w-0 space-y-12">
          <section aria-labelledby="recipe-steps" className="space-y-8">
            <h2
              id="recipe-steps"
              className="font-heading scroll-mt-8 text-2xl font-semibold"
            >
              烹饪步骤
            </h2>
            {steps.length > 0 ? (
              <ol className="space-y-10">
                {steps.map((step, index) => (
                  <li
                    key={`${step.number}-${index}`}
                    className="border-border space-y-5 border-b pb-10 last:border-0 last:pb-0"
                  >
                    <div className="flex items-center gap-3">
                      <span
                        aria-hidden="true"
                        className="bg-primary/10 text-primary flex size-10 shrink-0 items-center justify-center rounded-full text-base font-semibold"
                      >
                        {step.number}
                      </span>
                      <h3 className="text-base font-semibold">
                        步骤 {step.number}
                      </h3>
                    </div>
                    {step.text.trim() && (
                      <p className="text-base leading-8 break-words whitespace-pre-wrap">
                        {step.text}
                      </p>
                    )}
                    {step.image_urls.length > 0 && (
                      <div className="space-y-4">
                        {step.image_urls.map((url, imageIndex) => (
                          <RecipeImage
                            key={`${url}-${imageIndex}`}
                            src={url}
                            alt={`${recipe.title}，步骤 ${step.number} 配图 ${imageIndex + 1}`}
                          />
                        ))}
                      </div>
                    )}
                  </li>
                ))}
              </ol>
            ) : (
              <p className="text-muted-foreground text-base">
                原菜谱未提供烹饪步骤。
              </p>
            )}
          </section>

          {recipe.tips.trim() && (
            <section
              aria-labelledby="recipe-tips"
              className="border-primary/20 bg-primary/5 rounded-2xl border p-6 md:p-8"
            >
              <h2
                id="recipe-tips"
                className="font-heading text-2xl font-semibold"
              >
                小贴士
              </h2>
              <p className="mt-4 text-base leading-8 break-words whitespace-pre-wrap">
                {recipe.tips}
              </p>
            </section>
          )}
        </div>
      </div>
    </article>
  );
}

function RecipeRequest({
  id,
  loadRecipes,
  onRetry,
}: {
  id: string;
  loadRecipes: RecipeListLoader;
  onRetry: () => void;
}) {
  const [state, setState] = useState<RequestState>({ status: "loading" });

  useEffect(() => {
    const controller = new AbortController();
    const timeout = window.setTimeout(() => {
      controller.abort();
      setState({
        status: "error",
        title: "请求超时",
        message: "请求失败，请重试。",
      });
    }, 15000);
    window.scrollTo({ top: 0 });

    async function load() {
      try {
        const data = await loadRecipes(controller.signal);
        if (controller.signal.aborted) return;
        if (
          typeof data !== "object" ||
          data === null ||
          !("items" in data) ||
          !Array.isArray(data.items) ||
          !data.items.every(isRecipe) ||
          !("total" in data) ||
          data.total !== data.items.length
        ) {
          throw new Error("请求失败，请重试。");
        }
        const recipe = data.items.find((item: Recipe) => item.id === id);
        setState(
          recipe
            ? { status: "success", recipe }
            : {
                status: "error",
                title: "没有找到这道菜谱",
                message: "菜谱可能不存在，请检查链接中的菜谱 ID，或返回首页。",
              },
        );
      } catch (error) {
        if (!controller.signal.aborted) {
          setState({
            status: "error",
            title: "菜谱加载失败",
            message:
              error instanceof Error ? error.message : "请求失败，请重试。",
          });
        }
      } finally {
        window.clearTimeout(timeout);
      }
    }

    void load();
    return () => {
      controller.abort();
      window.clearTimeout(timeout);
    };
  }, [id, loadRecipes]);

  if (state.status === "loading")
    return <Loading message="正在加载菜谱…" className="min-h-96" />;
  if (state.status === "error")
    return (
      <ErrorState
        title={state.title}
        message={state.message}
        onRetry={onRetry}
        retryLabel="重新加载"
        className="min-h-96"
      />
    );
  return <RecipeContent recipe={state.recipe} />;
}

export default function RecipeDetail({
  loadRecipes = loadApiRecipes,
}: {
  loadRecipes?: RecipeListLoader;
}) {
  const { id = "" } = useParams();
  const [attempt, setAttempt] = useState(0);

  return (
    <div className="bg-background text-foreground min-h-svh font-sans">
      <header className="border-border border-b">
        <div className="mx-auto flex max-w-6xl items-center justify-between gap-4 px-6 py-5 md:px-10">
          <Link
            to="/"
            className="font-heading focus-visible:ring-ring/50 inline-flex items-center gap-2 rounded-md text-2xl font-semibold outline-none focus-visible:ring-3"
          >
            <ChefHat
              aria-hidden="true"
              className="text-primary size-7 shrink-0"
            />
            Recipe Finder
          </Link>
        </div>
      </header>
      <main className="mx-auto max-w-6xl px-6 pt-6 pb-16 md:px-10 md:pb-24">
        <Link
          to="/"
          className="text-muted-foreground hover:text-primary focus-visible:ring-ring/50 mb-8 inline-flex items-center gap-2 rounded-md text-sm outline-none focus-visible:ring-3"
        >
          <ArrowLeft aria-hidden="true" className="size-4" />
          返回首页
        </Link>
        <RecipeRequest
          key={`${id}-${attempt}`}
          id={id}
          loadRecipes={loadRecipes}
          onRetry={() => setAttempt((value) => value + 1)}
        />
      </main>
    </div>
  );
}
