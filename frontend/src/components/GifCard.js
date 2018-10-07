
import React, { Component } from 'react'
import { Card, Icon, Image, Label } from 'semantic-ui-react'

// Labels ---------------------------------------------------------------------

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

const copyToClipboard = str => {
  const el = document.createElement('textarea');
  el.value = str;
  document.body.appendChild(el);
  el.select();
  document.execCommand('copy');
  document.body.removeChild(el);
};

class ExpandModal extends Component {
  render () {
    return (
      <a class='ui label' style={ {float: 'left', textAlign: 'center'} } href={ this.props.url } download>
        <i className='bars icon' />
      </a>
    )
  }
}

class DownloadLabel extends Component {
  constructor (props) {
    super(props)
  }

  render () {
    return (
      <a class='ui label' style={ {float: 'right', textAlign: 'center'} } href={ this.props.url } download>
        <i className='save icon' />
      </a>
    )
  }
}

class CopyLinkLabel extends Component {
  constructor (props) {
    super(props)
    this.copyOnClick = this.copyOnClick.bind(this)
  }

  copyOnClick() {
    copyToClipboard(this.props.url)
  }

  render () {
    return (
      <a class='ui label' style={ {float: 'right', textAlign: 'center'} } onClick={ this.copyOnClick }>
        <i className='copy icon' />
      </a>
    )
  }
}

// Card -----------------------------------------------------------------------

// ExpandModal is commented out for now. Modals/specific gif views aren't implemented yet,
// but are a critical feature
export class GifCard extends Component {
  render () {
    let typeLabel = (this.props.ftype === 'Webm') ? (<WebmLabel />) : (<GifLabel />)
    let medial_link = '/data/' + this.props.fname
    return (
      <Card>
        <Image src={ medial_link } />
        <Card.Content extra>
          <ExpandModal />
          <DownloadLabel url={ window.location.origin + medial_link } />
          <CopyLinkLabel url={ window.location.origin + medial_link } />
        </Card.Content>
      </Card>
    )
  }
}
