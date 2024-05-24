
// LIST COMPONENT
/******************************************************************************/
function List({ list, onRemoveItem }) {
  return (
    <ul>
      {list.map((item) => (
        <Item key={item.objectId} item={item} onRemoveItem={onRemoveItem} />
      ))}
    </ul>
  );
}

// ITEM COMPONENT
/******************************************************************************/
function Item({ item, onRemoveItem }) {
  return (
    <li>
      <span>{item.title}</span>
      <br />
      <span>
        <a target="_blank" href={item.url}>
          {item.url}
        </a>
      </span>
      <br />
      <span>Author: {item.author}</span>
      <br />
      <span>Num Comments: </span>
      <span>{item.num_comments}</span>
      <br />
      <span>Num Points: </span>
      <span>{item.points}</span>
      <br />
      <span>
        <button type="button" onClick={() => onRemoveItem(item)}>
          Dismiss
        </button>
      </span>
    </li>
  );
}

export { List };
