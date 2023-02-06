import styles from './styles.scss';

const AccordionButton = ({ onClick, expanded, buttonText, dataTestId }) => {
  const mySpan = (
    <span data-testid="span_test_id" className="bold-text">
      {buttonText}
    </span>
  );

  return (
    <button
      className={`${styles.accordionButton}`}
      type="button"
      data-testid={dataTestId}
      data-cy="cy_test_id"
      onClick={onClick}
      aria-expanded={expanded}
    >
      <input type="text" />
      <span></span>
      <span>{buttonText}</span>
      <span aria-checked="true" data-testid="other_span_test_id">
        {buttonText}
      </span>
      {mySpan}
      <span
        data-testid={`
          ${dataTestId}_arrow
        `}
        className={`${styles.arrow} ${
          expanded ? 'icon-outline-up' : 'icon-outline-down'
        }`}
      />
      <MyComponent data-testid="mycomponent_id" />
      <MyComponent data-testid="mycomponent_id_2" />
      <MyLastComponent disabled />
    </button>
  );
};

export default AccordionButton;
