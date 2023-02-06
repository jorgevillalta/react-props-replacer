import styles from './styles.scss';

const AccordionButton = ({ onClick, expanded, buttonText, dataTestId }) => {
  const mySpan = <span className="bold-text">{buttonText}</span>;

  return (
    <button
      className={`${styles.accordionButton}`} type="button" onClick={onClick}
      aria-expanded={expanded}
    ><input type="text" />
      <span></span><span>{buttonText}</span><span aria-checked="true">{buttonText}</span>
      {mySpan}
      <span
        className={`${styles.arrow} ${
          expanded ? 'icon-outline-up' : 'icon-outline-down'
        }`}
      />
      <MyComponent data-testid={dataTestId} /><MyComponent data-testid="mycomponent_id_2" />
      <MyLastComponent disabled />
    </button>
  );
};

export default AccordionButton;