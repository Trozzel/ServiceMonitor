import * as React from 'react';

// INPUT COMPONENT WITH LABEL
/******************************************************************************/
function InputWithLabel({
  id,
  value,
  type = "text",
  onInputChange,
  isFocussed,
  children,
}) {
  const inputRef = React.useRef();

  React.useEffect(() => {
    if (isFocussed && inputRef.current) {
      inputRef.current.focus();
    }
  }, [isFocussed]);

  return (
    <>
      <label htmlFor={id}>{children}</label>
      <input id={id} type={type} value={value} onChange={onInputChange} />
      <hr />
      <p></p>
    </>
  );
}

export { InputWithLabel };
