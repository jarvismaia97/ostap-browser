import { useState } from "react";
import { motion, AnimatePresence } from "framer-motion";
import { Globe, PanelLeft, Bot, Sparkles, ArrowRight, Check } from "lucide-react";

interface Props {
  onComplete: () => void;
}

const steps = [
  {
    icon: Globe,
    title: "Welcome to Ostap",
    description: "A minimal browser for the focused mind. Fast, clean, and distraction-free.",
    accent: "from-purple-500 to-indigo-600",
  },
  {
    icon: PanelLeft,
    title: "Sidebar Tabs",
    description: "Your tabs live on the left — vertical, collapsible, and always within reach. Click the arrow to expand or collapse.",
    accent: "from-indigo-500 to-blue-600",
  },
  {
    icon: Bot,
    title: "Meet Jarvis",
    description: "Your AI assistant lives in the browser. Summarize pages, ask questions, or just chat — all without leaving your flow.",
    accent: "from-blue-500 to-cyan-600",
  },
  {
    icon: Sparkles,
    title: "You're all set",
    description: "Start browsing. Press ⌘T for a new tab, ⌘L to focus the address bar, or click the 🤖 to summon Jarvis.",
    accent: "from-cyan-500 to-emerald-600",
  },
];

export default function Onboarding({ onComplete }: Props) {
  const [step, setStep] = useState(0);
  const current = steps[step];
  const isLast = step === steps.length - 1;
  const Icon = current.icon;

  return (
    <div className="fixed inset-0 z-50 bg-bg/95 backdrop-blur-xl flex items-center justify-center">
      <div className="w-full max-w-md px-6">
        {/* Progress dots */}
        <div className="flex justify-center gap-2 mb-8">
          {steps.map((_, i) => (
            <div
              key={i}
              className={`h-1.5 rounded-full transition-all duration-300 ${
                i === step ? "w-8 bg-accent" : i < step ? "w-1.5 bg-accent/50" : "w-1.5 bg-border"
              }`}
            />
          ))}
        </div>

        <AnimatePresence mode="wait">
          <motion.div
            key={step}
            initial={{ opacity: 0, y: 20 }}
            animate={{ opacity: 1, y: 0 }}
            exit={{ opacity: 0, y: -20 }}
            transition={{ duration: 0.3 }}
            className="text-center"
          >
            {/* Icon */}
            <div className={`w-20 h-20 mx-auto mb-6 rounded-2xl bg-gradient-to-br ${current.accent} flex items-center justify-center shadow-lg shadow-accent/20`}>
              <Icon size={36} className="text-white" />
            </div>

            {/* Title */}
            <h2 className="text-2xl font-semibold text-txt mb-3">{current.title}</h2>

            {/* Description */}
            <p className="text-txt-secondary text-sm leading-relaxed max-w-sm mx-auto mb-10">
              {current.description}
            </p>

            {/* Button */}
            <button
              onClick={() => (isLast ? onComplete() : setStep(step + 1))}
              className="inline-flex items-center gap-2 px-6 py-3 bg-accent text-white rounded-xl text-sm font-medium hover:opacity-90 transition-opacity"
            >
              {isLast ? (
                <>
                  <Check size={16} />
                  Start Browsing
                </>
              ) : (
                <>
                  Continue
                  <ArrowRight size={16} />
                </>
              )}
            </button>

            {/* Skip */}
            {!isLast && (
              <button
                onClick={onComplete}
                className="block mx-auto mt-4 text-xs text-txt-secondary hover:text-txt transition-colors"
              >
                Skip intro
              </button>
            )}
          </motion.div>
        </AnimatePresence>
      </div>
    </div>
  );
}
