export const getExactPosition = (element: HTMLElement) => {
  let { x, y } = element.getBoundingClientRect();

  return {
    top: y + window.scrollY,
    left: x + window.scrollX,
  };
};
