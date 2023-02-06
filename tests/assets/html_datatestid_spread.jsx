import clsx from 'clsx';

import style from './styles.scss';

const Card = ({
  imgSrc,
  imgAlt,
  children,
  disabled = false,
  sideContent,
  skeleton = false,
  dataTestId,
  size = 's',
  ...otherProps
}) =>
  skeleton ? (
    <div size={size} />
  ) : (
    <div
      className={clsx(
        style.card,
        disabled && style.cardDisabled,
        style[`card--size-${size}`]
      )}
      {...(dataTestId && { 'data-testid': dataTestId })}
      {...otherProps}
    >
      <div className={style.imageContainer}>
        <span
          src={imgSrc}
          alt={imgAlt}
          {...(dataTestId && { 'data-testid': `${dataTestId}-img` })}
        />
      </div>
      <div className={style.contentContainer}>{children}</div>
      {sideContent && <div className={style.sideContainer}>{sideContent}</div>}
    </div>
  );

export default Card;
