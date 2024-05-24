import { InputWithLabel } from "./InputWithLabel";

// SEARCH FORM COMPONENT
/******************************************************************************/
function SearchForm({ searchTerm, onSearchInput, onSearchSubmit }) {
  return (
    <form onSubmit={onSearchSubmit}>
      <InputWithLabel
        id="search"
        value={searchTerm}
        isFocussed
        onInputChange={onSearchInput}
      >
        <strong>Search:</strong>
      </InputWithLabel>

      <button type="submit" disabled={!searchTerm}>
        Submit
      </button>
    </form>
  );
}

export { SearchForm };
