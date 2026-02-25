import React from "react";

interface BadgeProps {
  type: "add" | "sub" | "mod";
  count: number;
}

const Badge: React.FC<BadgeProps> = ({ type, count }) => {
  const config = {
    add: { bg: "bg-green-100", text: "text-green-700", label: "+" },
    sub: { bg: "bg-red-100", text: "text-red-700", label: "-" },
    mod: { bg: "bg-blue-100", text: "text-blue-700", label: "~" },
  };

  const { bg, text, label } = config[type];

  return (
    <span
      className={`inline-flex items-center px-2 py-0.5 rounded text-xs font-medium ${bg} ${text} mr-2`}
    >
      {label}
      {count}
    </span>
  );
};

export default Badge;
