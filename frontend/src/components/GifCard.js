
import React, { Component } from 'react'
import { Card, Icon, Image, Label } from 'semantic-ui-react'

const WebmLabel = () => (
  <Label>Webm</Label>

)
const GifLabel = () => (
  <Label>Gif</Label>
)

const CaptionLabel = () => (
  <Label>
    <Icon name='closed captioning' />
  </Label>
)

class ViewsLabel extends Component {
  render () {
    return (
      <Label>
        <Icon name='eye' />{this.props.value}
      </Label>
    )
  }
}

export class GifCard extends Component {
  render () {
    let typeLabel = (this.props.ftype === 'Webm') ? (<WebmLabel />) : (<GifLabel />)
    return (
      <Card>
        <Image src={'data/' + this.props.fname} />
        <Card.Content>
          <Card.Header>{this.props.title}</Card.Header>
          {/* <Card.Meta>
            <a>Spongebob</a>
            <a>Patrick</a>
          </Card.Meta> */}
          <Card.Description>{this.props.description}</Card.Description>
        </Card.Content>
        <Card.Content extra>
          <ViewsLabel value={this.props.views} />
          {/* <CaptionLabel /> */}
          { typeLabel }
        </Card.Content>
      </Card>
    )
  }
}
