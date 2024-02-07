import torch
from torch import nn

class Foo(nn.Module):
  def __init__(self):
    super(Foo, self).__init__()
    self.conv = nn.Sequential(nn.Conv2d(3, 6, 5))

model = Foo()

torch.save(model.state_dict(), "foo.pt")
