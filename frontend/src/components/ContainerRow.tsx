import {Container} from '../types';

export type ContainerRowProps = {
    container: Container,
};

export const ContainerRow = (props: ContainerRowProps) => {
    return (
        <div>
            ----------- Container: {props.container.name} ({ props.container.usage?.cpu } {props.container.usage?.memory})
        </div>
    );
};