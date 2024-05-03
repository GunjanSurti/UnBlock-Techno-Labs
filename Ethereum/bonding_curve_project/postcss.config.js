module.exports = {
  plugins: {
    tailwindcss: {},
    autoprefixer: {},
  },
  extends: "stylelint-config-standard-scss",
  rules: {
    "function-disallowed-list": ["/^rgb/", "/^hsl/"],
    "color-named": "never",
    "selector-pseudo-element-no-unknown": [
      true,
      { ignorePseudoElements: ["ng-deep"] },
    ],
    "selector-class-pattern":
      "^([a-z][a-z0-9]*)(-[a-z0-9]+)*$|^noUi-|calenderMatMenu",
  },
};
